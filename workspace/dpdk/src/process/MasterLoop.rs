// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.




// TODO: Service core configuration in DpdkConfiguration
	// See evt_service_setup in evt_common.h for an example of 'balancing' services and assigned a service (service_id) to a service logical core.
// TODO: How to stop control threads and service cores???
	/*
		rte_service_component_runstate_set(t->tx_service.service_id, 0);
		rte_service_runstate_set(t->tx_service.service_id, 0);
		rte_service_component_unregister(t->tx_service.service_id);
	*/
// TODO: Logical core choices.
// TODO: Initialize logical slave cores and services..
	// TODO: Explore rte_eventdev

// TODO: Stop all ethernet devices and queues - could be quite interesting.
// TODO: Run custom services on service cores.


// TODO: PacketBufferExt needs finishing
// various methods
// fragmentation outward, inward
// GRO for TCP / IPv4

// TODO: Mellanox Performance Tuning: https://community.mellanox.com/docs/DOC-2489



/// Master loop.
pub struct MasterLoop
{
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

macro_rules! wait_for_signals
{
	($self: ident, $signals_to_wait_for: ident, $running_interactively: ident) =>
	{
		{
			use self::TimedSignalWait::*;
			
			match one_millisecond_timed_wait_for_signals(&$signals_to_wait_for)
			{
				TimedOut => (),
				
				OtherSignalInterrupted =>
				{
					$self.should_function_terminate.exit_signalled(None);
					
					return None
				}
				
				Signalled(signal_number) =>
				{
					$self.should_function_terminate.exit_signalled(Some(signal_number));
					
					return if $running_interactively
					{
						match signal_number
						{
							SIGTERM => None,
							SIGHUP => None,
							SIGINT => Some(SIGINT),
							SIGQUIT => Some(SIGQUIT),
							
							_ => panic!("Blocked signal '{}' received", signal),
						}
					}
					else
					{
						match signal_number
						{
							SIGTERM => None,
							
							_ => panic!("Blocked signal '{}' received", signal),
						}
					}
				}
			}
		}
	}
}

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
	/// The `hybrid_global_allocator` should be declared globally in `main.rs` as `#[global_allocator] static ALLOCATOR: HybridGlobalAllocator = HybridGlobalAllocator::new();`.
	///
	/// The number of logical cores used is calculated by examining the Linux command line kernel parameters.
	///
	/// It is recommended that Linux run with at least 2 cores assigned to the Kernel; one of these will be used as a master logical core, and the other will be used for control threads as necessary. Neither usage is particularly high or critical.
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
		master_loop_configuration.set_maximum_resource_limits();
		
		master_loop_configuration.load_kernel_modules();
		
		master_loop_configuration.write_system_control_values();
		
		let cpu_features = master_loop_configuration.validate_minimal_cpu_features();
		
		let isolated_hyper_threads_including_those_offline = master_loop_configuration.validate_kernel_command_line(&cpu_features);
		
		let (online_shared_hyper_threads, online_isolated_hyper_threads) = master_loop_configuration.online_shared_and_isolated_hyper_threads(isolated_hyper_threads_including_those_offline);
		
		let master_logical_core = master_loop_configuration.find_master_logical_core_and_tell_linux_to_use_shared_hyper_threads_for_all_needs(&online_shared_hyper_threads);
		
		let (slave_logical_cores, service_logical_cores) = master_loop_configuration.divide_logical_cores_into_slave_logical_cores_and_service_logical_cores(online_isolated_hyper_threads);
		
		let (hugetlbfs_mount_path, memory_limits) = master_loop_configuration.configure_huge_pages();
		
		let pci_devices_and_original_driver_names = master_loop_configuration.pci_devices_and_original_driver_names();
		
		let success_or_failure = catch_unwind(|| self.execute_after_pci_devices_bound_to_drivers(master_loop_configuration, hybrid_global_allocator, &pci_devices_and_original_driver_names, hugetlbfs_mount_path, memory_limits, master_logical_core, &slave_logical_cores, &service_logical_cores));
		
		PciNetDevicesConfiguration::release_all_from_use_with_dpdk(&master_loop_configuration.path_configuration.sys_path, pci_devices_and_original_driver_names);
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_pci_devices_bound_to_drivers(&self, master_loop_configuration: &MasterLoopConfiguration, hybrid_global_allocator: &HybridGlobalAllocator, pci_devices: &HashMap<PciDevice, Option<String>>, hugetlbfs_mount_path: PathBuf, memory_limits: MachineOrNumaNodes<MegaBytes>, master_logical_core: HyperThread, slave_logical_cores: &BTreeSet<HyperThread>, service_logical_cores: &BTreeSet<HyperThread>)
	{
		MasterLoopConfiguration::block_all_signals_before_initializing_dpdk_so_that_slave_logical_cores_do_not_handle_signals();
		
		master_loop_configuration.initialize_dpdk(hybrid_global_allocator, pci_devices, &hugetlbfs_mount_path, memory_limits, master_logical_core, slave_logical_cores, service_logical_cores);
		
		let slave_logical_cores_to_uses = master_loop_configuration.slave_logical_cores_to_uses(pci_devices, slave_logical_cores);
		
		let success_or_failure = catch_unwind(|| self.execute_after_dpdk_initialized(master_loop_configuration, &slave_logical_cores_to_uses));
		
		DpdkConfiguration::dpdk_clean_up();
		hybrid_global_allocator.dpdk_was_cleaned_up();
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_dpdk_initialized(&self, master_loop_configuration: &MasterLoopConfiguration, slave_logical_cores_to_uses: &HashMap<LogicalCore, Box<Fn(LogicalCore, &Arc<ShouldFunctionTerminate>)>>) -> Option<SignalNumber>
	{
		master_loop_configuration.lock_down_security();
		
		let logical_core_power_managers = master_loop_configuration.logical_core_power_to_maximum();
		
		let success_or_failure = catch_unwind(||
		{
			for slave_logical_core in LogicalCore::slave_logical_cores_without_service_cores()
			{
				(slave_logical_cores_to_uses.remove(&slave_logical_core).unwrap())(slave_logical_core, &self.should_function_terminate)
			}
			self.progress_busy_loop_with_signal_handling(master_loop_configuration)
		});
		
		#[inline(always)]
		fn clean_up(logical_core_power_managers: Vec<LogicalCorePowerManagement>)
		{
			LogicalCore::block_until_all_slaves_are_in_the_wait_state();
			MasterLoopConfiguration::restore_default_power(logical_core_power_managers)
		}
		
		match success_or_failure
		{
			Err(panic_payload) =>
			{
				self.should_function_terminate.we_panicked(&panic_payload);
				
				clean_up(logical_core_power_managers);
				
				resume_unwind(panic_payload)
			}
			
			Ok(reraise_signal) =>
			{
				clean_up(logical_core_power_managers);
				
				reraise_signal
			}
		}
	}
	
	#[inline(always)]
	fn progress_busy_loop_with_signal_handling(&self, master_loop_configuration: &MasterLoopConfiguration) -> Option<SignalNumber>
	{
		let mut timer_progress_engine = master_loop_configuration.timer_progress_engine();
		
		let running_interactively = master_loop_configuration.running_interactively();
		
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
		
		while self.should_function_terminate.should_continue()
		{
			timer_progress_engine.progress();
			
			wait_for_signals!(self, signals_to_wait_for, running_interactively)
		}
	}
}
