// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A wrapper type to catch panics and ensure termination.
#[derive(Debug)]
pub struct PanicCatchingSlaveLogicalCoreFunction
{
	function_which_can_panic: ServiceFunction,
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

impl SlaveLogicalCoreFunction for PanicCatchingSlaveLogicalCoreFunction
{
	#[inline(always)]
	fn execute(&mut self)
	{
		let can_panic = &mut self.function_which_can_panic;
		let success_or_failure = catch_unwind(AssertUnwindSafe(|| can_panic.execute()));
		
		if let Err(panicked_with) = success_or_failure
		{
			self.should_function_terminate.we_panicked(panicked_with.as_ref())
		}
	}
}

impl PanicCatchingSlaveLogicalCoreFunction
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(function_which_can_panic: ServiceFunction, should_function_terminate: &Arc<ShouldFunctionTerminate>) -> Self
	{
		Self
		{
			function_which_can_panic,
			should_function_terminate: should_function_terminate.clone(),
		}
	}
}

/// A wrapper type to catch panics and ensure termination.
#[derive(Debug)]
pub struct PanicCatchingServiceFunction
{
	function_which_can_panic: ServiceFunction,
	should_function_terminate: Arc<ShouldFunctionTerminate>,
	panicked: bool,
}

impl ServiceFunction for PanicCatchingServiceFunction
{
	#[inline(always)]
	fn execute(&mut self)
	{
		if unlikely(self.panicked)
		{
			return
		}
		
		let can_panic = &mut self.function_which_can_panic;
		let success_or_failure = catch_unwind(AssertUnwindSafe(|| can_panic.execute()));
		
		if let Err(panicked_with) = success_or_failure
		{
			self.should_function_terminate.we_panicked(panicked_with.as_ref())
		}
	}
}

impl PanicCatchingServiceFunction
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(can_panic: ServiceFunction, should_function_terminate: &Arc<ShouldFunctionTerminate>) -> Self
	{
		Self
		{
			function_which_can_panic,
			should_function_terminate: should_function_terminate.clone(),
			panicked: false,
		}
	}
}



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

// TODO: Mellanox Performance Tuning: https://community.mellanox.com/docs/DOC-2489

impl MasterLoop
{
	/// An exit code that is for normal software exits.
	pub const EXIT_SUCCESS: i32 = 0;
	
	/// An exit code that is for software failures (ie panics).
	pub const EX_SOFTWARE: i32 = 70;
	
	/// Creates a new instance.
	#[cold]
	pub fn new(hybrid_global_allocator: &HybridGlobalAllocator) -> Self
	{
		Self
		{
			should_function_terminate: ShouldFunctionTerminate::new()
		}
	}
	
	/// Executes a program which uses DPDK.
	///
	/// The `hybrid_global_allocator` should be declared globally in `main.rs` as `#[global_allocator] static ALLOCATOR: HybridGlobalAllocator = HybridGlobalAllocator::new();`
	///
	/// If running interactively `SIGINT` and `SIGQUIT` are intercepted and will be re-raised (using libc's `raise()`) after handling so that any parent shell can behave correctly.
	///
	/// Always returns normally; panics are handled and logged.
	///
	/// The return value is an exit code in the range 0 - 127 inclusive which should be passed to `std::process::exit()`. Currenty values are:-
	///
	/// * `0`: successful
	/// * `70`: (aka `EX_SOFTWARE`) - something panicked
	///
	/// Notes:-
	///
	/// * The daemon `irqbalance` should not really be run when this program is running. It isn't incompatible per se, but it isn't useful.
	/// * It is recommended to boot the kernel with the command line parameter `irqaffinity` set to the inverse of `isolcpus`.
	/// * If running causes Linux Kernel modules to load, these are **not** unloaded at process exit as we no longer have the permissions to do so,
	/// * Likewise, if we mount `hugeltbfs` it is not unmounted (and, if we created its mount point folder, this is not deleted) at process exit.
	#[cold]
	pub fn execute(&self, master_loop_configuration: &MasterLoopConfiguration) -> i32
	{
		master_loop_configuration.start_logging();
		
		let exit_code_or_error = catch_unwind(||
		{
			let reraise_signal = master_loop_configuration.daemonize_if_required(|| self.execute_after_daemonizing(master_loop_configuration, hybrid_global_allocator));
			
			if let Some(reraise_signal_number) = reraise_signal
			{
				master_loop_configuration.stop_logging();
				unsafe { raise(reraise_signal_number) };
			}
			
			Self::EXIT_SUCCESS
		});
		
		master_loop_configuration.stop_logging();
		
		match exit_code_or_error
		{
			Ok(exit_code) => exit_code,
			Err(panicked_with) =>
			{
				LoggingConfiguration::caught_unwind(panicked_with.as_ref());
				
				Self::EX_SOFTWARE
			}
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
