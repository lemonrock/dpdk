// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Whilst kernel modules may be loaded by this process, we do not unload them on process exit.
///
/// This is because at this stage we typically won't have permissions to do so.
///
// NOTE: We do not try to unmount or remove our hugetlbfs mount point as this is too brittle.
// NOTE: We do not try to unload and loaded modules, likewise because it is too brittle.
pub struct MasterLoop
{
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

// TODO: Unmount hugetlbfs mount; decide if we should inherit existing mounts.
// TODO: Revise memory core mapping in DpdkConfiguration - use hyper threads.
// TODO:HyperThread and related types which aren't finished.
// TODO: PacketBufferExt needs finishing
	// various methods
	// fragmentation outward
// TODO: Understand who needs a service core.
// TODO: Initialize slave logical cores (and service cores).
// TODO: Set termination signal for all logical cores.
// TODO: Wait for all logical cores
	// TODO: but handle if a core unexpectedly panics (wrap all logic in a thread handler).
// TODO: Stop all ethernet devices and queues.
// TODO: Service core configuration in DpdkConfiguration
// TODO: Logical core choices.`
// TODO: Incorporate knowledge of hyper-thread siblings using eg `sys/devices/system/cpu/cpu0/cache/index0/shared_cpu_list` (which works better than [hyper] thread_siblings_list on virtualized systems, eg Parallels)
	// If being a performance pedant, avoid using logical cores with a hyper-thread sibling.

// ?/sys/devices/virtual/workqueue/cpumask ?

impl MasterLoop
{
	/// Executes.
	///
	/// Panics may be caught but are re-raised.
	///
	/// If running interactively (`daemonize == None`), then `SIGINT` and `SIGQUIT` are intercepted and will be re-raised if caught so that any parent shell can behave correctly.
	#[inline(always)]
	pub fn execute(&self, path_configuration: &PathConfiguration, dpdk_configuration: &DpdkConfiguration, pci_net_devices_configuration: &PciNetDevicesConfiguration, daemonize: Option<Daemonize>, warnings_to_suppress: &WarningsToSuppress)
	{
		let reraise_signal = if let Some(daemonize) = daemonize
		{
			let daemonize_clean_up_on_exit = daemonize.daemonize();
			let success_or_failure = catch_unwind(|| self.execute_after_daemonizing(path_configuration, dpdk_configuration, pci_net_devices_configuration, false));
			daemonize_clean_up_on_exit.clean_up();
			
			match success_or_failure
			{
				Err(failure) => resume_unwind(failure),
				Ok(reraise_signal) => reraise_signal,
			}
		}
		else
		{
			self.execute_after_daemonizing(path_configuration, dpdk_configuration, pci_net_devices_configuration, true)
		};
		
		if let Some(reraise_signal_number) = reraise_signal
		{
			unsafe { raise(reraise_signal_number) };
		}
	}
	
	#[inline(always)]
	fn execute_after_daemonizing(&self, path_configuration: &PathConfiguration, dpdk_configuration: &DpdkConfiguration, pci_net_devices_configuration: &PciNetDevicesConfiguration, running_interactively: bool) -> Option<SignalNumber>
	{
		const UsesPowerManagement: bool = true;
		let cpu_features = CpuFeatures::validate_minimal_cpu_features(warnings_to_suppress, UsesPowerManagement);
		
		// TODO: Use this to calculate logical cpus.
		let isolated_hyper_threads = KernelCommandLineValidator::validate(path_configuration, warnings_to_suppress, &cpu_features, pci_net_devices_configuration);
		
		self.set_maximum_resource_limits();
		
		let (hugetlbfs_mount_path, memory_limits) = Self::configure_huge_pages(path_configuration, dpdk_configuration);
		
		self.load_kernel_modules(path_configuration, dpdk_configuration, pci_net_devices_configuration);
		
		let pci_devices_and_original_driver_names = pci_net_devices_configuration.take_for_use_with_dpdk(&path_configuration.sys_path);
		
		let success_or_failure = catch_unwind(|| self.execute_after_pci_devices_bound_to_drivers(path_configuration, dpdk_configuration, &pci_devices_and_original_driver_names, hugetlbfs_mount_path, memory_limits, running_interactively));
		
		PciNetDevicesConfiguration::release_all_from_use_with_dpdk(&path_configuration.sys_path, pci_devices_and_original_driver_names);
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_pci_devices_bound_to_drivers(&self, path_configuration: &PathConfiguration, dpdk_configuration: &DpdkConfiguration, pci_devices: &HashMap<PciDevice, Option<String>>, hugetlbfs_mount_path: PathBuf, memory_limits: MachineOrNumaNodes<MegaBytes>, running_interactively: bool)
	{
		Self::block_all_signals_before_initializing_dpdk_so_that_slave_logical_cores_do_not_handle_signals();
		
		dpdk_configuration.initialize_dpdk(pci_devices, &hugetlbfs_mount_path, memory_limits).expect("Could not initialize DPDK");
		
		let success_or_failure = catch_unwind(|| self.execute_after_dpdk_initialized(dpdk_configuration, running_interactively));
		
		Self::dpdk_clean_up();
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_dpdk_initialized(&self, dpdk_configuration: &DpdkConfiguration, running_interactively: bool) -> Option<SignalNumber>
	{
		dpdk_configuration.enable_high_precision_event_timer_after_dpdk_initialized_if_configured();
		
		Self::initialize_dpdk_timer_subsystem();
		
		fn initialize_dpdk_timer_subsystem()
		{
			unsafe { rte_timer_subsystem_init() };
		}
		
		Self::remove_nearly_all_capabilities();
		
		Self::disable_dumpable();
		
		Self::no_new_privileges();
		
		Self::clear_all_ambient_capabilities();
		
		Self::lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();
		
		// TODO: Initialize logical slave cores and services..
		// TODO - control threads by default run on the first 'ROLE_OFF' core; if there are none of these, then they run on the master core.
		
		let reraise_signal = Self::infinite_signal_handling_and_timer_progress_loop(running_interactively);
		
		// TODO: Set termination signal for all logical cores.
		
		// TODO: Wait for all logical cores
		
		// TODO: Stop all ethernet devices.
		
		reraise_signal
	}
	
	#[inline(always)]
	fn set_maximum_resource_limits(&self)
	{
		ResourceLimitsSet::defaultish(ResourceLimit::maximum_number_of_open_file_descriptors(&self.proc_path).expect("Could not read maximum number of file descriptors"));
	}
	
	#[inline(always)]
	fn configure_huge_pages(path_configuration: &PathConfiguration, dpdk_configuration: &DpdkConfiguration) -> (PathBuf, MachineOrNumaNodes<MegaBytes>)
	{
		let huge_page_mount_settings = &dpdk_configuration.huge_page_mount_settings;
		let huge_page_allocation_strategy = &dpdk_configuration.huge_page_allocation_strategy;
		let sys_path = &path_configuration.sys_path;
		
		Self::disable_transparent_huge_pages();
		
		path_configuration.proc_path.filesystems().unwrap().verify_hugetlbfs_is_supported();
		
		let mounts = path_configuration.proc_path.mounts().unwrap();
		let (unmount, hugetlbfs_mount_path) = match mounts.existing_hugetlbfs_mount()
		{
			Some(hugetlbfs_mount_path) => (None, hugetlbfs_mount_path.to_owned()),
			None(_) => (Some(huge_page_mount_settings.mount(sys_path)), huge_page_mount_settings.mount_point.to_owned())
		};
		
		let machine_or_numa_nodes = MachineOrNumaNodes::new(sys_path);
		machine_or_numa_nodes.garbage_collect_memory(sys_path);
		
		let memory_limits = NumaNodeChoice::reserve_huge_page_memory(&path_configuration.sys_path, &path_configuration.proc_path, huge_page_allocation_strategy);
		
		(hugetlbfs_mount_path, memory_limits)
	}
	
	#[inline(always)]
	fn load_kernel_modules(&self, path_configuration: &PathConfiguration, dpdk_configuration: &DpdkConfiguration, pci_net_devices_configuration: &PciNetDevicesConfiguration)
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			let mut essential_kernel_modules = Vec::new();
			if dpdk_configuration.has_kernel_native_interface_virtual_devices()
			{
				essential_kernel_modules.push(EssentialKernelModule::RteKni);
			}
			pci_net_devices_configuration.add_essential_kernel_modules(&mut essential_kernel_modules);
			
			let mut modules_loaded = path_configuration.proc_path.modules();
			let mut essential_kernel_modules_to_unload = EssentialKernelModulesToUnload::new();
			for essential_kernel_module in essential_kernel_modules.iter()
			{
				essential_kernel_module.load_if_necesary(modules_loaded, &path_configuration.dpdk_provided_kernel_modules_path, &mut essential_kernel_modules_to_unload, &path_configuration: &PathConfiguration.dev_path);
			}
		}
	}
	
	#[inline(always)]
	fn block_all_signals_before_initializing_dpdk_so_that_slave_logical_cores_do_not_handle_signals()
	{
		block_all_signals_on_current_thread();
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
	
	#[inline(always)]
	fn remove_nearly_all_capabilities()
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			use self::Capability::*;
			
			const CapabilitiesToDrop: [Capability; 34] =
			[
				AuditControl,
				AuditRead,
				AuditWrite,
				BlockSuspend,
				Chown,
				DiscretionaryAccessControlBypass,
				DiscretionaryAccessControlFileReadBypass,
				FileOwnerBypass,
				FileSetId,
				//LockMemory,
				IpcOwner,
				Kill,
				Lease,
				Immutable,
				MandatoryAccessControlBypass,
				MandatoryAccessControlOverride,
				MakeNodes,
				SystemAdministration,
				NetworkAdministration,
				BindPortsBelow1024,
				//NetRaw,
				SetUid,
				SetGid,
				SetFileCapabilities,
				SetProcessCapabilities,
				RebootAndKexecLoad,
				Chroot,
				KernelModule,
				Nice,
				ProcessAccounting,
				PTrace,
				RawIO,
				Resource,
				Time,
				TtyConfig,
				Syslog,
				WakeAlarm,
			];
			
			Capability::ensure_capabilities_dropped(&CapabilitiesToDrop);
		}
	}
	
	#[inline(always)]
	fn disable_transparent_huge_pages()
	{
		adjust_transparent_huge_pages(false)
	}
	
	#[inline(always)]
	fn disable_dumpable()
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			disable_dumpable();
		}
	}
	
	#[inline(always)]
	fn no_new_privileges()
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			no_new_privileges();
		}
	}
	
	#[inline(always)]
	fn clear_all_ambient_capabilities()
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			Capability::clear_all_ambient_capabilities();
		}
	}
	
	#[inline(always)]
	fn lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities()
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();
		}
	}
	
	#[inline(always)]
	fn dpdk_clean_up()
	{
		unsafe { rte_eal_cleanup() };
	}
}
