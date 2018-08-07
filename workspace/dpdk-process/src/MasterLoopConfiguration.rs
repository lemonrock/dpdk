// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Configuration holder.
#[derive(Debug)]
#[derive(Deserialize)]
#[serde(default)]
pub struct MasterLoopConfiguration
{
	/// DPDK configuration.
	pub dpdk_configuration: DpdkConfiguration,
	
	/// PCI network devices to use.
	pub pci_net_devices_configuration: PciNetDevicesConfiguration,
	
	/// Number of service cores to use. Defaults to 1.
	pub service_cores: u8,
	
	/// Logging configuration.
	pub logging_configuration: LoggingConfiguration,
	
	/// Should we daemonize? (Default, yes).
	pub daemonize: Option<Daemonize>,
	
	/// System control settings (`sysctl`).
	///
	/// By default turns off swapping.
	pub system_control_settings: HashMap<String, u64>,
	
	/// Suppress any unwanted warnings about ideal CPU features or the Linux Kernel command line parameters.
	pub warnings_to_suppress: WarningsToSuppress,
	
	/// Enables power management; forces all logical cores to TurboBoost if possible.
	pub power_to_maximum: bool,
	
	/// Number of cycles to wait before checking timers on master loop.
	///
	/// Defaults to a value equivalent to 100 milliseconds at 2Ghz.
	pub timer_progress_engine_cycles: Cycles,
	
	/// Location of `/dev`, `/proc` and `/sys`.
	pub path_configuration: PathConfiguration,
}

impl Default for MasterLoopConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			dpdk_configuration: DpdkConfiguration::default(),
			
			pci_net_devices_configuration: PciNetDevicesConfiguration::default(),
			
			service_cores: 1,
			
			logging_configuration: LoggingConfiguration::default(),
			
			daemonize: Some(Daemonize::default()),
			
			system_control_settings: hashmap!
			{
				"vm.swappiness".to_string() => 0,
				"vm.zone_reclaim_mode".to_string() => 0,
				"vm.dirty_ratio".to_string() => 10,
				"vm.dirty_background_ratio".to_string() => 5,
			},
			
			warnings_to_suppress: WarningsToSuppress::default(),
			
			power_to_maximum: true,
			
			timer_progress_engine_cycles: Cycles::AroundTenMillisecondsAt2GigaHertzSuitableForATimerProgressEngine,
			
			path_configuration: PathConfiguration::default(),
		}
	}
}

impl MasterLoopConfiguration
{
	#[inline(always)]
	pub(crate) fn start_logging(&self)
	{
		self.logging_configuration.configure_rust_stack_back_traces();
		self.logging_configuration.configure_syslog(self.running_interactively());
		self.logging_configuration.configure_panic_hook()
	}
	
	#[inline(always)]
	pub(crate) fn daemonize_if_required<F: Fn() -> Option<SignalNumber>>(&self, execute_after_daemonizing: F) -> Option<SignalNumber>
	{
		if let Some(daemonize) = self.daemonize
		{
			let daemonize_clean_up_on_exit = daemonize.daemonize();
			let success_or_failure = catch_unwind(AssertUnwindSafe(|| execute_after_daemonizing()));
			daemonize_clean_up_on_exit.clean_up();
			
			match success_or_failure
			{
				Err(failure) =>
				{
					self.stop_logging();
					
					resume_unwind(failure)
				}
				Ok(reraise_signal) => reraise_signal,
			}
		}
		else
		{
			execute_after_daemonizing()
		}
	}
	
	#[inline(always)]
	pub(crate) fn stop_logging(&self)
	{
		self.logging_configuration.stop_panic_hook();
		self.logging_configuration.stop_logging()
	}
	
	#[inline(always)]
	pub(crate) fn validate_minimal_cpu_features(&self) -> CpuFeatures
	{
		CpuFeatures::validate_minimal_cpu_features(&self.warnings_to_suppress, self.power_to_maximum)
	}
	
	#[inline(always)]
	pub(crate) fn validate_kernel_command_line(&self, cpu_features: &CpuFeatures) -> BTreeSet<HyperThread>
	{
		KernelCommandLineValidator::validate(&self.path_configuration, &self.warnings_to_suppress, cpu_features, &self.pci_net_devices_configuration)
	}
	
	#[inline(always)]
	pub(crate) fn online_shared_and_isolated_hyper_threads(&self, isolated_hyper_threads_including_those_offline: BTreeSet<HyperThread>) -> (BTreeSet<HyperThread>, BTreeSet<HyperThread>)
	{
		assert_ne!(isolated_hyper_threads_including_those_offline.len(), 0, "There must be at least one hyper thread in `isolated_hyper_threads_including_those_offline`");
		
		let shared_hyper_threads_including_those_offline = HyperThread::complement(&isolated_hyper_threads_including_those_offline, self.sys_path());
		assert_ne!(shared_hyper_threads_including_those_offline.len(), 0, "There must be at least one hyper thread in `shared_hyper_threads_including_those_offline`");
		
		let online_isolated_hyper_threads = HyperThread::remove_those_offline(&isolated_hyper_threads_including_those_offline, self.sys_path());
		assert_ne!(online_isolated_hyper_threads.len(), 0, "There must be at least one hyper thread in `online_isolated_hyper_threads`");
		
		let online_shared_hyper_threads = HyperThread::remove_those_offline(&shared_hyper_threads_including_those_offline, self.sys_path());
		assert_ne!(online_shared_hyper_threads.len(), 0, "There must be at least one hyper thread in `online_shared_hyper_threads`");
		
		self.warnings_to_suppress.miscellany_warn("too_many_shared_hyper_threads", "There are more than 2 shared hyper threads", || online_shared_hyper_threads.len() <= 2);
		self.warnings_to_suppress.miscellany_warn("too_few_shared_hyper_threads", "There is only 1 shared hyper thread (which will be shared with the master logical core and control threads)", || online_shared_hyper_threads.len() != 1);
		
		{
			let mut numa_nodes = BTreeSet::new();
			if self.sys_path().is_a_numa_machine()
			{
				for online_shared_hyper_thread in online_shared_hyper_threads.iter()
				{
					let insert = (*online_shared_hyper_thread).numa_node(self.sys_path()).unwrap();
					numa_nodes.insert(insert);
				}
				self.warnings_to_suppress.miscellany_warn("too_many_numa_nodes_shared_hyper_threads", &format!("More than one (actually, {:?}) NUMA nodes are present in the shared hyper threads", numa_nodes), || numa_nodes.len() == 1);
			}
		}
		
		{
			for hyper_thread_group in HyperThread::hyper_thread_groups(&online_shared_hyper_threads, self.sys_path()).iter()
			{
				let mut hits = 0;
				for hyper_thread in hyper_thread_group.iter()
				{
					if online_shared_hyper_threads.contains(hyper_thread)
					{
						hits += 1;
					}
				}
				self.warnings_to_suppress.miscellany_warn("overlapping_shared_hyper_threads", &format!("More than one (actually, {}) hyper threads of the group '{:?}' are present in the shared hyper threads", hits, hyper_thread_group), || hits < 2);
			}
		}
		
		(online_shared_hyper_threads, online_isolated_hyper_threads)
	}
	
	#[inline(always)]
	pub(crate) fn find_master_logical_core_and_tell_linux_to_use_shared_hyper_threads_for_all_needs(&self, online_shared_hyper_threads: &BTreeSet<HyperThread>) -> HyperThread
	{
		let master_logical_core = HyperThread::last(online_shared_hyper_threads).unwrap();
		
		InterruptRequest::force_all_interrupt_requests_to_just_these_hyper_threads(online_shared_hyper_threads, self.proc_path());
		
		HyperThread::set_work_queue_hyper_thread_affinity(online_shared_hyper_threads, self.sys_path());
		
		HyperThread::force_watchdog_to_just_these_hyper_threads(online_shared_hyper_threads, self.proc_path());
		
		*master_logical_core
	}
	
	#[inline(always)]
	pub(crate) fn divide_logical_cores_into_slave_logical_cores_and_service_logical_cores(&self, isolated_hyper_threads: BTreeSet<HyperThread>) -> (BTreeSet<HyperThread>, BTreeSet<HyperThread>)
	{
		assert!(isolated_hyper_threads.len() > self.service_cores(), "There must be more isolated hyper threads '{}' than number of service cores '{}'", isolated_hyper_threads.len(), self.service_cores());
		
		// TODO: It's very difficult to decide on a good strategy for service cores, but they should not be hyper thread siblings; possibly they should come from a different NUMA node to master.
		
		// rte_event_eth_rx_adapter uses a poll over devices and ports, `event_eth_rx_adapter_service_func()` - and wrr - weighted round robin - to 'over balance' some rx queue - ethernet socket combinations. See eth_poll_wrr_calc() for calculaiton of polling weights (more entries for more weight, scaled down to minimal array size using GCD, greatest common deniminator).
		
		// See also `sw_event_timer_adapter_service_func` which takes in messages to arm or cancel using `rte_timer_reset`.
		
		xxxx;
		unimplemented!()
	}
	
	#[inline(always)]
	pub(crate) fn slave_logical_cores_to_uses(&self, pci_devices: &HashMap<PciDevice, Option<String>>, slave_logical_cores: &BTreeSet<HyperThread>) -> HashMap<LogicalCore, Box<Fn(LogicalCore, &Arc<ShouldFunctionTerminate>)>>
	{
		// TODO: Probably some sort of 2-loop strategy, the first to work out ethernet device capabilities (we will need somewhere to stick some configuration), the second to assign processes.
		// We need to reserve some space for UCX processes. Everything else can either run on master or a service core.
		// Our minimal spec - 4 cores - leaves just one core available as a slave logical core...

		// We should look to shard Rx / Tx logic so similar code runs on similar hyper-thread pairs so that the L1 instruction cache is best used.
		
		/*
			Example of Box<Fn(LogicalCore, &Arc<ShouldFunctionTerminate>)>:-
			
			/// Very irritatingly, we can't return the result of `create_some_busy_poll_behaviour_type_a()` without it being a boxed trait object.
			fn xxx(slave_logical_core: LogicalCore, should_function_terminate: &Arc<ShouldFunctionTerminate>)
			{
				create_some_busy_poll_behaviour_type_a().execute_code_on_slave(slave_logical_core, should_function_terminate)
			}
		*/
		xxxx;
		unimplemented!()
	}
	
	#[inline(always)]
	pub(crate) fn set_maximum_resource_limits(&self)
	{
		ResourceLimitsSet::defaultish(ResourceLimit::maximum_number_of_open_file_descriptors(self.proc_path()).expect("Could not read maximum number of file descriptors"));
	}
	
	#[inline(always)]
	pub(crate) fn configure_huge_pages(&self) -> (PathBuf, Option<MachineOrNumaNodes<MegaBytes>>)
	{
		let huge_page_mount_settings = &self.dpdk_configuration.huge_page_mount_settings;
		let huge_page_allocation_strategy = &self.dpdk_configuration.huge_page_allocation_strategy;
		
		self.disable_transparent_huge_pages();
		
		self.path_configuration.proc_path.filesystems().unwrap().verify_hugetlbfs_is_supported();
		
		let mounts = self.path_configuration.proc_path.mounts().unwrap();
		let (unmount, hugetlbfs_mount_path) = match mounts.existing_hugetlbfs_mount()
		{
			None => (Some(huge_page_mount_settings.mount(self.sys_path())), huge_page_mount_settings.mount_point.to_owned()),
			Some(hugetlbfs_mount_path) => (None, hugetlbfs_mount_path.to_owned())
		};
		
		let machine_or_numa_nodes = MachineOrNumaNodes::new(self.sys_path());
		machine_or_numa_nodes.garbage_collect_memory(self.sys_path());
		
		let memory_limits = match self.dpdk_configuration.huge_page_allocation_strategy
		{
			None => None,
			Some(ref huge_page_allocation_strategy) => Some(machine_or_numa_nodes.reserve_huge_page_memory(self.sys_path(), self.proc_path(), huge_page_allocation_strategy).unwrap())
		};
		
		(hugetlbfs_mount_path, memory_limits)
	}
	
	#[cfg(target_os = "linux")]
	#[inline(always)]
	pub(crate) fn load_kernel_modules(&self) -> EssentialKernelModulesToUnload
	{
		let mut essential_kernel_modules = HashSet::new();
		if self.dpdk_configuration.has_kernel_native_interface_virtual_devices()
		{
			essential_kernel_modules.insert(EssentialKernelModule::RteKni);
		}
		self.pci_net_devices_configuration.add_essential_kernel_modules(&mut essential_kernel_modules);
		
		let mut modules_loaded = self.path_configuration.proc_path.modules().unwrap();
		let mut essential_kernel_modules_to_unload = EssentialKernelModulesToUnload::new();
		for essential_kernel_module in essential_kernel_modules.iter()
		{
			essential_kernel_module.load_if_necesary(&mut modules_loaded, &self.path_configuration.dpdk_provided_kernel_modules_path, &mut essential_kernel_modules_to_unload);
		}
		
		essential_kernel_modules_to_unload
	}
	
	#[inline(always)]
	pub(crate) fn write_system_control_values(&self)
	{
		self.proc_path().write_system_control_values(&self.system_control_settings).unwrap()
	}
	
	#[inline(always)]
	pub(crate) fn pci_devices_and_original_driver_names(&self) -> HashMap<PciDevice, Option<String>>
	{
		self.sys_path().rescan_all_pci_buses_and_devices();
		
		self.pci_net_devices_configuration.take_for_use_with_dpdk(self.sys_path())
	}
	
	#[inline(always)]
	pub(crate) fn block_all_signals_before_initializing_dpdk_so_that_slave_logical_cores_do_not_handle_signals()
	{
		block_all_signals_on_current_thread();
	}
	
	#[inline(always)]
	pub(crate) fn initialize_dpdk<V>(&self, hybrid_global_allocator: &'static HybridGlobalAllocator, pci_devices: &HashMap<PciDevice, V>, hugetlbfs_mount_path: &Path, memory_limits: Option<MachineOrNumaNodes<MegaBytes>>, master_logical_core: HyperThread, slave_logical_cores: &BTreeSet<HyperThread>, service_logical_cores: &BTreeSet<HyperThread>)
	{
		self.dpdk_configuration.initialize_dpdk(&self.logging_configuration, pci_devices, &hugetlbfs_mount_path, memory_limits, master_logical_core, slave_logical_cores, service_logical_cores);
		
		hybrid_global_allocator.dpdk_is_now_configured();
		
		self.dpdk_configuration.enable_high_precision_event_timer_after_dpdk_initialized_if_configured();
		
		fn initialize_dpdk_timer_subsystem()
		{
			unsafe { rte_timer_subsystem_init() };
		}
		
		initialize_dpdk_timer_subsystem();
	}
	
	#[inline(always)]
	pub(crate) fn logical_core_power_to_maximum(&self) -> Vec<LogicalCorePowerManagement>
	{
		if self.power_to_maximum
		{
			let logical_core_power_managers = Vec::with_capacity(LogicalCore::number_of_logical_cores());
			
			for logical_core in LogicalCore::all_logical_cores()
			{
				if let Ok(logical_core_power_management) = logical_core.start_power_management()
				{
					logical_core_power_management.enable_turbo_boost();
					logical_core_power_management.set_to_maximum_frequency();
					logical_core_power_managers.push(logical_core_power_management);
				}
			}
			
			logical_core_power_managers
		}
		else
		{
			Default::default()
		}
	}
	
	#[inline(always)]
	pub(crate) fn restore_default_power(logical_core_power_managers: Vec<LogicalCorePowerManagement>)
	{
		drop(logical_core_power_managers);
	}
	
	#[inline(always)]
	pub(crate) fn lock_down_security(&self)
	{
		#[cfg(target_os = "linux")]
		{
			use self::Capability::*;
			
			Capability::ensure_capabilities_dropped
			(&[
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
			]);
			
			disable_dumpable();
			
			no_new_privileges();
			
			Capability::clear_all_ambient_capabilities();
			
			lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();
		}
	}
	
	#[inline(always)]
	pub(crate) fn timer_progress_engine(&self) -> TimerProgressEngine
	{
		TimerProgressEngine::new(self.timer_progress_engine_cycles)
	}
	
	#[inline(always)]
	pub(crate) fn running_interactively(&self) -> bool
	{
		self.daemonize.is_none()
	}
	
	#[inline(always)]
	fn disable_transparent_huge_pages(&self)
	{
		self.sys_path().change_transparent_huge_pages_defragmentation(TransparentHugePageDefragmentationChoice::Never, 4096, 60_000, 10_000, 511, 64);
		self.sys_path().change_transparent_huge_pages_usage(TransparentHugePageRegularMemoryChoice::Never, TransparentHugePageSharedMemoryChoice::Never, true);
		
		const EnableHugeTransparentPages: bool = false;
		adjust_transparent_huge_pages(EnableHugeTransparentPages);
	}
	
	#[inline(always)]
	fn service_cores(&self) -> usize
	{
		self.service_cores as usize
	}
	
	#[inline(always)]
	fn proc_path(&self) -> &ProcPath
	{
		&self.path_configuration.proc_path
	}
	
	#[inline(always)]
	fn sys_path(&self) -> &SysPath
	{
		&self.path_configuration.sys_path
	}
}
