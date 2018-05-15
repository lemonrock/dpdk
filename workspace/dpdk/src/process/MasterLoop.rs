// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Master loop.
pub struct MasterLoop
{
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

// TODO: PacketBufferExt needs finishing
	// various methods
	// fragmentation outward
// TODO: Set termination signal for all logical cores.
// TODO: Wait for all logical cores
	// TODO: but handle if a core unexpectedly panics (wrap all logic in a thread handler).
// TODO: Stop all ethernet devices and queues.

// TODO: Understand who needs a service core.
// TODO: Initialize slave logical cores (and service cores).

// TODO: Service core configuration in DpdkConfiguration
// TODO: Logical core choices.

// TODO: Incorporate knowledge of hyper-thread siblings using eg `sys/devices/system/cpu/cpu0/cache/index0/shared_cpu_list` (which works better than [hyper] thread_siblings_list on virtualized systems, eg Parallels).
	// We should look to shard Rx / Tx logic so similar code runs on similar hyper-thread pairs so that the L1 instruction cache is best used.

// TODO: Bitch about:-
	// More than one reserved CPU (or pair of hyper-threads for the same CPU) per NUMA node?
	// default_smp_affinity != irqaffinity on kernel command line?

// TODO: Do we always want to set the socket-memory ?
	// perhaps we want it uncapped; perhaps we don;t always want to garbage collect, etc

// TODO: Do we want to use libc's malloc and friends (eg aligned_alloc) for HybridGlobalAllocator
	// - we need a way to track whose memory is whose.
	// - we can use unsafe { rte_malloc_validate(pointer, null() } == -1  to detect non-DPDK memory, as the cost of making realloc / free more expensive than current.
	// musl doesn't seem to provide a way to detect if memory was allocated by it.
	// We could use increased allocations by abusing size / alignment
		// eg if asked to allocate 256 bytes, 16 byte aligned we allocate 272 bytes, reserving 16 bytes for ourselves.
		// eg we could use jemalloc.

// TODO: Mellanox Performance Tuning: https://community.mellanox.com/docs/DOC-2489

impl MasterLoop
{
	/// Executes a program which uses DPDK.
	///
	/// Panics may be caught but are re-raised.
	///
	/// If running interactively `SIGINT` and `SIGQUIT` are intercepted and will be re-raised if caught so that any parent shell can behave correctly.
	///
	/// The `hybrid_global_allocator` should be declared globally in `main.rs` as `#[global_allocator] static ALLOCATOR: HybridGlobalAllocator = HybridGlobalAllocator::new();`
	///
	/// Notes:-
	///
	/// * The daemon `irqbalance` should not really be run when this program is running. It isn't incompatible per se, but it isn't useful.
	/// * It is recommended to boot the kernel with the command line parameter `irqaffinity` set to the inverse of `isolcpus`.
	/// * If running causes Linux Kernel modules to load, these are **not** unloaded at process exit as we no longer have the permissions to do so,
	/// * Likewise, if we mount `hugeltbfs` it is not unmounted (and, if we created its mount point folder, this is not deleted) at process exit.
	#[inline(always)]
	pub fn execute(&self, master_loop_configuration: &MasterLoopConfiguration, hybrid_global_allocator: &HybridGlobalAllocator)
	{
		let reraise_signal = if let Some(daemonize) = daemonize
		{
			let daemonize_clean_up_on_exit = daemonize.daemonize();
			let success_or_failure = catch_unwind(|| self.execute_after_daemonizing(master_loop_configuration, hybrid_global_allocator));
			daemonize_clean_up_on_exit.clean_up();
			
			match success_or_failure
			{
				Err(failure) => resume_unwind(failure),
				Ok(reraise_signal) => reraise_signal,
			}
		}
		else
		{
			self.execute_after_daemonizing(master_loop_configuration, hybrid_global_allocator)
		};
		
		if let Some(reraise_signal_number) = reraise_signal
		{
			unsafe { raise(reraise_signal_number) };
		}
	}
	
	#[inline(always)]
	fn execute_after_daemonizing(&self, master_loop_configuration: &MasterLoopConfiguration, hybrid_global_allocator: &HybridGlobalAllocator) -> Option<SignalNumber>
	{
		master_loop_configuration.load_kernel_modules();
		
		master_loop_configuration.write_system_control_values();
		
		let cpu_features = master_loop_configuration.validate_minimal_cpu_features();
		
		let isolated_hyper_threads = master_loop_configuration.validate_kernel_command_line_and_return_isolated_hyper_threads(&cpu_features);
		
		let master_hyper_thread = master_loop_configuration.find_master_hyper_thread_and_tell_linux_to_use_shared_hyper_threads_for_all_needs(&isolated_hyper_threads);
		
		master_loop_configuration.set_maximum_resource_limits();
		
		let (hugetlbfs_mount_path, memory_limits) = configure_huge_pages.configure_huge_pages();
		
		let pci_devices_and_original_driver_names = master_loop_configuration.pci_devices_and_original_driver_names();
		
		let success_or_failure = catch_unwind(|| self.execute_after_pci_devices_bound_to_drivers(master_loop_configuration, hybrid_global_allocator, &pci_devices_and_original_driver_names, hugetlbfs_mount_path, memory_limits, master_hyper_thread, &isolated_hyper_threads));
		
		PciNetDevicesConfiguration::release_all_from_use_with_dpdk(&master_loop_configuration.path_configuration.sys_path, pci_devices_and_original_driver_names);
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_pci_devices_bound_to_drivers(&self, master_loop_configuration: &MasterLoopConfiguration, hybrid_global_allocator: &HybridGlobalAllocator, pci_devices: &HashMap<PciDevice, Option<String>>, hugetlbfs_mount_path: PathBuf, memory_limits: MachineOrNumaNodes<MegaBytes>, master_logical_core: HyperThread, remaining_logical_cores: &BTreeSet<HyperThread>)
	{
		MasterLoopConfiguration::block_all_signals_before_initializing_dpdk_so_that_slave_logical_cores_do_not_handle_signals();
		
		master_loop_configuration.initialize_dpdk(hybrid_global_allocator, pci_devices, &hugetlbfs_mount_path, memory_limits, master_logical_core, remaining_logical_cores);
		
		let success_or_failure = catch_unwind(|| self.execute_after_dpdk_initialized(master_loop_configuration));
		
		DpdkConfiguration::dpdk_clean_up();
		hybrid_global_allocator.dpdk_was_cleaned_up();
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_dpdk_initialized(&self, master_loop_configuration: &MasterLoopConfiguration) -> Option<SignalNumber>
	{
		master_loop_configuration.lock_down_security();
		
		let logical_core_power_managers = master_loop_configuration.logical_core_power_to_maximum();
		
		// TODO: Initialize logical slave cores and services..
		// TODO - control threads by default run on the first 'ROLE_OFF' core; if there are none of these, then they run on the master core.
		
		let reraise_signal = Self::infinite_signal_handling_and_timer_progress_loop(master_loop_configuration.running_interactively());
		
		MasterLoopConfiguration::restore_default_power(logical_core_power_managers);
		
		// TODO: Set termination signal for all logical cores.
		
		// TODO: Wait for all logical cores
		
		// TODO: Stop all ethernet devices.
		
		reraise_signal
	}
	
	#[inline(always)]
	fn infinite_signal_handling_and_timer_progress_loop(running_interactively: bool) -> Option<SignalNumber>
	{
		let mut timer_progress_engine = TimerProgressEngine::new(Cycles::AroundTenMillisecondsAt2GigaHertzSuitableForATimerProgressEngine);
		
		let signals_to_accept = if running_interactively
		{
			hashset!
			{
				SIGTERM,
				SIGHUP,
				SIGINT,
				SIGQUIT,
			}
		}
		else
		{
			hashset!
			{
				SIGTERM,
				// NOTE: `SIGHUP` has been used conventionally to force a daemon to re-read its configuration; we're probably better off using `SIGUSR1` or `SIGUSR2`.
				// `SIGUSR1` / `SIGUSR2` can also be used, with `sigqueue`, to send a 32-bit value to a process using `SI_QUEUE` `si_code`.
			}
		};
		
		block_all_signals_on_current_thread_bar(&signals_to_accept);
		
		let signals_to_wait_for = hash_set_to_signal_set(&signals_to_accept);
		
		loop
		{
			timer_progress_engine.progress();
			
			use self::TimedSignalWait::*;
			match one_millisecond_timed_wait_for_signals(&signals_to_wait_for)
			{
				TimedOut => continue,
				
				Signalled(signal_number) => if running_interactively
				{
					match signal_number
					{
						SIGTERM => return None,
						SIGHUP => return None,
						SIGINT => return Some(SIGTERM),
						SIGQUIT => return Some(SIGQUIT),
						
						_ => panic!("Blocked signal '{}' received", signal),
					}
				}
				else
				{
					match signal_number
					{
						SIGTERM => return None,
						
						_ => panic!("Blocked signal '{}' received", signal),
					}
				}
				
				OtherSignalInterrupted => return None,
			}
		}
	}
}
