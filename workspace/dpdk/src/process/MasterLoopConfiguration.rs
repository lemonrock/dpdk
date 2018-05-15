// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


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
			
			pci_net_devices_configuration: PciNetDevicesConfiguration,
			
			daemonize: Some(Daemonize::default()),
			
			system_control_settings: hashmap!
			{
				"vm.swappiness" => 0
				"vm.zone_reclaim_mode" => 0
				"vm.dirty_ratio" => 10
				"vm.dirty_background_ratio" => 5,
			},
			
			warnings_to_suppress: WarningsToSuppress::default(),
			
			path_configuration: PathConfiguration::default(),
		}
	}
}

impl MasterLoopConfiguration
{
	#[inline(always)]
	pub(crate) fn validate_minimal_cpu_features(&self) -> CpuFeatures
	{
		CpuFeatures::validate_minimal_cpu_features(&self.warnings_to_suppress, self.power_to_maximum)
	}
	
	#[inline(always)]
	pub(crate) fn validate_kernel_command_line_and_return_isolated_hyper_threads(&self) -> BTreeSet<HyperThread>
	{
		let isolated_hyper_threads = KernelCommandLineValidator::validate(&self.path_configuration, &self.warnings_to_suppress, &cpu_features, &self.pci_net_devices_configuration);
		assert_ne!(isolated_hyper_threads.len(), 0, "There are no `isolcpus` on the Linux kernel command line (isolated CPUs)");
		
		isolated_hyper_threads
	}
	
	#[inline(always)]
	pub(crate) fn find_master_hyper_thread_and_tell_linux_to_use_shared_hyper_threads_for_all_needs(&self, isolated_hyper_threads: &BTreeSet<HyperThread>) -> HyperThread
	{
		let all_hyper_threads = HyperThread::online(self.sys_path());
		let shared_hyper_threads: BTreeSet<HyperThread> =  all_hyper_threads.difference(isolated_hyper_threads).cloned().collect();
		self.proc_path().force_all_interrupt_requests_to_just_these_hyper_threads(shared_hyper_threads);
		self.sys_path().set_work_queue_cpu_affinity(shared_hyper_threads);
		self.proc_path().force_watchdog_to_just_these_hyper_threads(shared_hyper_threads);
		let master_hyper_thread = shared_hyper_threads.iter().rev().next();
		master_hyper_thread
	}
	
	#[inline(always)]
	pub(crate) fn set_maximum_resource_limits(&self)
	{
		ResourceLimitsSet::defaultish(ResourceLimit::maximum_number_of_open_file_descriptors(self.proc_path()).expect("Could not read maximum number of file descriptors"));
	}
	
	#[inline(always)]
	pub(crate) fn configure_huge_pages(&self) -> (PathBuf, MachineOrNumaNodes<MegaBytes>)
	{
		let huge_page_mount_settings = &self.dpdk_configuration.huge_page_mount_settings;
		let huge_page_allocation_strategy = &self.dpdk_configuration.huge_page_allocation_strategy;
		
		self.disable_transparent_huge_pages();
		
		path_configuration.proc_path.filesystems().unwrap().verify_hugetlbfs_is_supported();
		
		let mounts = self.path_configuration.proc_path.mounts().unwrap();
		let (unmount, hugetlbfs_mount_path) = match mounts.existing_hugetlbfs_mount()
		{
			Some(hugetlbfs_mount_path) => (None, hugetlbfs_mount_path.to_owned()),
			None(_) => (Some(huge_page_mount_settings.mount(self.sys_path())), huge_page_mount_settings.mount_point.to_owned())
		};
		
		let machine_or_numa_nodes = MachineOrNumaNodes::new(self.sys_path());
		machine_or_numa_nodes.garbage_collect_memory(self.sys_path());
		
		let memory_limits = NumaNodeChoice::reserve_huge_page_memory(self.sys_path(), self.proc_path(), huge_page_allocation_strategy);
		
		(hugetlbfs_mount_path, memory_limits)
	}
	
	#[cfg(target_os = "linux")]
	#[inline(always)]
	pub(crate) fn load_kernel_modules(&self) -> EssentialKernelModulesToUnload
	{
		let mut essential_kernel_modules = Vec::new();
		if self.dpdk_configuration.has_kernel_native_interface_virtual_devices()
		{
			essential_kernel_modules.push(EssentialKernelModule::RteKni);
		}
		self.pci_net_devices_configuration.add_essential_kernel_modules(&mut essential_kernel_modules);
		
		let mut modules_loaded = self.path_configuration.proc_path.modules();
		let mut essential_kernel_modules_to_unload = EssentialKernelModulesToUnload::new();
		for essential_kernel_module in essential_kernel_modules.iter()
		{
			essential_kernel_module.load_if_necesary(modules_loaded, &self.path_configuration.dpdk_provided_kernel_modules_path, &mut essential_kernel_modules_to_unload, &self.path_configuration.dev_path);
		}
		
		essential_kernel_modules_to_unload
	}
	
	#[inline(always)]
	pub(crate) fn write_system_control_values(&self)
	{
		self.proc_path().write_system_control_values(&self.system_control_settings)
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
	pub(crate) fn initialize_dpdk<V>(&self, pci_devices: &HashMap<PciDevice, V>, hugetlbfs_mount_path: &Path, memory_limits: MachineOrNumaNodes<MegaBytes>, master_logical_core: HyperThread, remaining_logical_cores: &BTreeSet<HyperThread>)
	{
		self.dpdk_configuration.initialize_dpdk(pci_devices, &hugetlbfs_mount_path, memory_limits, master_logical_core, remaining_logical_cores);
	}
	
	#[inline(always)]
	pub(crate) fn enable_dpdk_timer_logic(&self)
	{
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
				if let Some(logical_core_power_management) = logical_core.start_power_management()
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
	pub(crate) fn running_interactively(&self) -> bool
	{
		self.daemonize.is_none()
	}
	
	#[inline(always)]
	fn disable_transparent_huge_pages(&self)
	{
		self.sys_path().change_transparent_huge_pages_defragmentation(TransparentHugePageDefragmentation::never, 4096, 60_000, 10_000, 511, 64);
		self.sys_path().change_transparent_huge_pages_usage(TransparentHugePageRegularMemoryChoice::never, TransparentHugePageSharedMemoryChoice::never, true);
		
		const EnableHugeTransparentPages: bool = false;
		adjust_transparent_huge_pages(EnableHugeTransparentPages);
	}
	
	#[inline(always)]
	fn proc_path(&self) -> &Path
	{
		&self.path_configuration.proc_path
	}
	
	#[inline(always)]
	fn sys_path(&self) -> &Path
	{
		&self.path_configuration.sys_path
	}
}
