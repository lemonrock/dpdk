// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.




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


// TODO: Review PKT_RX_TIMESTAMP and whether to use 		ts = rte_get_tsc_cycles();
// m->timestamp = ts;
// m->ol_flags |= PKT_RX_TIMESTAMP;
// Also do_softrss / rte_softrss_be / rte_softrss (non-optimized), part of rte_thash (toeplitz hash functions), used entirely by the ethernet event adaptor.



// TODO: Scheduler.set_for_current_thread() for slave cores;


/// Master loop.
pub struct MasterLoop
{
	should_function_terminate: Arc<ShouldFunctionTerminate>,
	hybrid_global_allocator: &'static HybridGlobalAllocator,
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
							
							_ => panic!("Blocked signal '{}' received", signal_number),
						}
					}
					else
					{
						match signal_number
						{
							SIGTERM => None,
							
							_ => panic!("Blocked signal '{}' received", signal_number),
						}
					}
				}
			}
		}
	}
}

impl MasterLoop
{
	/// Creates a new instance.
	///
	/// The `hybrid_global_allocator` should be declared globally in `main.rs` as `#[global_allocator] static ALLOCATOR: HybridGlobalAllocator = HybridGlobalAllocator::new();`.
	#[cold]
	pub fn new(hybrid_global_allocator: &'static HybridGlobalAllocator) -> Self
	{
		Self
		{
			should_function_terminate: ShouldFunctionTerminate::new(),
			hybrid_global_allocator
		}
	}
	
	/// Executes a program which uses DPDK.
	///
	/// The number of logical cores used is calculated by examining the Linux command line kernel parameters.
	#[cold]
	pub fn execute(self, master_loop_configuration: &MasterLoopConfiguration) -> i32
	{
		master_loop_configuration.process_common_configuration.execute
		(
			||
			{
				master_loop_configuration.load_kernel_modules()
			},

			self.power_to_maximum,

			true,

			|linux_kernel_command_line_parameters|
			{
				let (uses_igb_uio, uses_vfio_pci) = self.pci_net_devices_configuration.uses_ugb_uio_or_pci_vfio();
				Self::validate_dpdk_pci_drivers(linux_kernel_command_line_parameters, uses_igb_uio, uses_vfio_pci)
			},

			|online_shared_hyper_threads_for_os, _online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core|
			{
				InterruptRequest::force_all_interrupt_requests_to_just_these_hyper_threads(online_shared_hyper_threads_for_os, self.proc_path())?;

				let (slave_logical_cores, service_logical_cores) = master_loop_configuration.divide_logical_cores_into_slave_logical_cores_and_service_logical_cores(online_isolated_hyper_threads_for_process);

				let (hugetlbfs_mount_path, memory_limits) = master_loop_configuration.configure_huge_pages()?;

				let pci_devices_and_original_driver_names = master_loop_configuration.pci_devices_and_original_driver_names();

				let success_or_failure = catch_unwind(AssertUnwindSafe(|| self.execute_after_pci_devices_bound_to_drivers(master_loop_configuration, &pci_devices_and_original_driver_names, hugetlbfs_mount_path, memory_limits, master_logical_core, &slave_logical_cores, &service_logical_cores)));

				PciNetDevicesConfiguration::release_all_from_use_with_dpdk(master_loop_configuration.sys_path(), pci_devices_and_original_driver_names);

				match success_or_failure
				{
					Err(failure) => resume_unwind(failure),
					Ok(reraise_signal) => Ok(reraise_signal),
				}
			},
		).unwrap()
	}

	#[inline(always)]
	fn validate_dpdk_pci_drivers(linux_kernel_command_line_parameters: &LinuxKernelCommandLineParameters, uses_igb_uio: bool, uses_vfio_pci: bool) -> Result<(), String>
	{
		macro_rules! fail
		{
			($message: literal) =>
			{
				return Err($message.to_string())
			}
		}

		if uses_igb_uio
		{
			if let Some(iommu_setting) = linux_kernel_command_line_parameters.iommu()
			{
				if iommu_setting != "pt"
				{
					fail!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu` is not `iommu=pt` (pass through)");
				}
			}
			else
			{
				fail!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu=pt` (pass through) was not specified");
			}

			if let Some(intel_iommu_setting) = linux_kernel_command_line_parameters.intel_iommu()
			{
				if intel_iommu_setting != "on"
				{
					fail!("Using igb_uio driver and iommu Linux Kernel command line parameter `intel_iommu` is not `intel_iommu=on`");
				}
			}
			else
			{
				fail!("Using igb_uio driver and iommu Linux Kernel command line parameter `intel_iommu=on` was not specified");
			}
		}

		if uses_vfio_pci
		{
			if let Some(iommu_setting) = linux_kernel_command_line_parameters.iommu()
			{
				if iommu_setting != "pt" || iommu_setting != "on"
				{
					fail!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu` is not `iommu=pt` (pass through) or `iommu=on`");
				}
			}
			else
			{
				fail!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu=pt` (pass through) or `iommu=on` was not specified");
			}
		}

		Ok(())
	}

	#[inline(always)]
	fn execute_after_pci_devices_bound_to_drivers(&self, master_loop_configuration: &MasterLoopConfiguration, pci_devices: &HashMap<PciDevice, Option<String>>, hugetlbfs_mount_path: PathBuf, memory_limits: Option<MachineOrNumaNodes<MegaBytes>>, master_logical_core: HyperThread, slave_logical_cores: &BTreeSet<HyperThread>, service_logical_cores: &BTreeSet<HyperThread>) -> Option<SignalNumber>
	{
		master_loop_configuration.initialize_dpdk(self.hybrid_global_allocator, pci_devices, &hugetlbfs_mount_path, memory_limits, master_logical_core, slave_logical_cores, service_logical_cores);
		
		let slave_logical_cores_to_uses = master_loop_configuration.slave_logical_cores_to_uses(pci_devices, slave_logical_cores);
		
		let success_or_failure = catch_unwind(AssertUnwindSafe(|| self.execute_after_dpdk_initialized(master_loop_configuration, &slave_logical_cores_to_uses)));
		
		DpdkConfiguration::dpdk_clean_up();
		self.hybrid_global_allocator.dpdk_was_cleaned_up();
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => return reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_dpdk_initialized(&self, master_loop_configuration: &MasterLoopConfiguration, slave_logical_cores_to_uses: &HashMap<LogicalCore, Box<Fn(LogicalCore, &Arc<ShouldFunctionTerminate>)>>) -> Option<SignalNumber>
	{
		ProcessCommonConfiguration::lock_down_security();
		
		let logical_core_power_managers = master_loop_configuration.logical_core_power_to_maximum();
		
		let success_or_failure = catch_unwind(AssertUnwindSafe(||
		{
			for slave_logical_core in LogicalCore::slave_logical_cores_without_service_cores()
			{
				(slave_logical_cores_to_uses.remove(&slave_logical_core).unwrap())(slave_logical_core, &self.should_function_terminate)
			}
			self.progress_busy_loop_with_signal_handling(master_loop_configuration)
		}));
		
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
		master_loop_configuration.adjust_scheduling().expect("Could not adjust scheduling for master logical core");

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
		
		None
	}
}
