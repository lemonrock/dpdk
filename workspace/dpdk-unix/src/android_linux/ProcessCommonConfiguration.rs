// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.
//


/// Process common configuration.
#[derive(Debug)]
#[derive(Deserialize)]
#[serde(default)]
pub struct ProcessCommonConfiguration
{
	/// Logging configuration.
	pub logging_configuration: LoggingConfiguration,

	/// Locale.
	pub locale: Box<[u8]>,

	/// Process niceness.
	pub process_niceness: ProcessNiceness,

	/// System control settings (`sysctl`).
	///
	/// By default turns off swapping.
	pub system_control_settings: HashMap<String, u64>,

	/// Suppress any unwanted warnings about ideal CPU features or the Linux Kernel command line parameters.
	pub warnings_to_suppress: WarningsToSuppress,

	/// Should we daemonize? (Default, yes).
	pub daemonize: Option<Daemonize>,

	/// Location of `/proc`.
	pub proc_path: ProcPath,

	/// Location of `/sys`.
	pub sys_path: SysPath,

	/// Location of `/dev`.
	pub dev_path: DevPath,
}

impl Default for ProcessCommonConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			logging_configuration: LoggingConfiguration::default(),

			locale: b"en_US.UTF-8\0".to_vec().into_boxed_slice(),

			process_niceness: ProcessNiceness::default(),

			system_control_settings:
			{
				let mut system_control_settings = HashMap::with_capacity(4);
				system_control_settings.insert("vm.swappiness".to_string(), 0);
				system_control_settings.insert("vm.zone_reclaim_mode".to_string(), 0);
				system_control_settings.insert("vm.dirty_ratio".to_string(), 10);
				system_control_settings.insert("vm.dirty_background_ratio".to_string(), 5);
				system_control_settings
			},

			warnings_to_suppress: WarningsToSuppress::default(),

			daemonize: Some(Daemonize::default()),

			proc_path: ProcPath::default(),

			sys_path: SysPath::default(),

			dev_path: DevPath::default(),
		}
	}
}

impl ProcessCommonConfiguration
{
	/// Executes a program.
	///
	/// It is recommended that Linux run with at least 2 cores assigned to the Kernel; one of these will be used as a master logical core, and the other will be used for control threads as necessary.
	/// Neither usage is particularly high or critical.
	///
	/// If running interactively `SIGINT` and `SIGQUIT` are intercepted and will be re-raised (using libc's `raise()`) after handling so that any parent shell can behave correctly.
	///
	/// Always returns normally; panics are handled and returned as `ProcessCommonConfigurationExecutionError::ExecutionPanicked`.
	///
	/// Notes:-
	///
	/// * The daemon `irqbalance` should not really be run when this program is running. It isn't incompatible per se, but it isn't useful.
	/// * It is recommended to boot the kernel with the command line parameter `irqaffinity` set to the inverse of `isolcpus`.
	/// * If running causes Linux Kernel modules to load, these are **not** unloaded at process exit as we no longer have the permissions to do so.
	/// * Likewise, if we mount `hugeltbfs` it is not unmounted (and, if we created its mount point folder, this is not deleted) at process exit.
	#[cold]
	pub fn execute(mut self, load_kernel_modules: impl Fn() -> Result<(), String>, uses_enhanced_intel_speedstep_technology: bool, isolated_cpus_required: bool, additional_kernel_command_line_validations: impl FnOnce(&LinuxKernelCommandLineParameters) -> Result<(), String>, main_loop: impl Fn(BTreeSet<HyperThread>, BTreeSet<HyperThread>, BTreeSet<HyperThread>, HyperThread) -> Result<Option<SignalNumber>, String>) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		self.start_logging();

		let result: ::std::thread::Result<Result<(), ProcessCommonConfigurationExecutionError>> = catch_unwind(AssertUnwindSafe(||
		{
			block_all_signals_on_current_thread();

			self.set_locale();

			let valid_hyper_threads_for_the_current_process = self.valid_hyper_threads_for_the_current_process();

			Self::set_current_process_affinity(&valid_hyper_threads_for_the_current_process)?;

			self.adjust_process_niceness()?;

			self.set_maximum_resource_limits();

			load_kernel_modules().map_err(|explanation| ProcessCommonConfigurationExecutionError::CouldNotLoadKernelModules(explanation))?;

			self.write_system_control_values()?;

			self.rescan_all_pci_buses_and_devices()?;

			let cpu_features = self.validate_minimal_cpu_features(uses_enhanced_intel_speedstep_technology)?;

			let isolated_hyper_threads_including_those_offline = self.validate_kernel_command_line(isolated_cpus_required, &cpu_features, additional_kernel_command_line_validations)?;

			let (online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core) = self.hyper_thread_sets(isolated_hyper_threads_including_those_offline);

			self.tell_linux_to_use_shared_hyper_threads_for_all_needs(&online_shared_hyper_threads_for_os)?;

			let reraise_signal = self.daemonize_if_required(main_loop, online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core)?;

			match reraise_signal
			{
				Some(reraise_signal_number) =>
				{
					self.stop_logging();
					unsafe { raise(reraise_signal_number) };
					Ok(())
				}

				None => Ok(()),
			}
		}));

		self.stop_logging();

		result?
	}

	/// * Removes most capabilities, except for those locking memory, binding ports, raw I/O, raw network ports and changing `nice` like values.
	/// * Disables dumpable.
	/// * Removes the right to new privileges.
	/// * Removes ambient capabilities.
	/// * Locks all secure bits and removes the ability to raise or keep ambient capabilities.
	#[inline(always)]
	pub fn lock_down_security()
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
			//BindPortsBelow1024,
			//NetRaw,
			SetUid,
			SetGid,
			SetFileCapabilities,
			SetProcessCapabilities,
			RebootAndKexecLoad,
			Chroot,
			KernelModule,
			//Nice,
			ProcessAccounting,
			PTrace,
			//RawIO,
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

	/// Removes the ability to lock memory.
	#[inline(always)]
	pub fn lock_down_security_after_memory_locked()
	{
		Capability::ensure_capabilities_dropped(&[Capability::LockMemory]);
	}

	/// Removes the ability to change a thread's nice value.
	#[inline(always)]
	pub fn lock_down_thread_nice_value_setting()
	{
		Capability::ensure_capabilities_dropped(&[Capability::Nice]);
	}

	/// Removes the ability to change a thread's nice value.
	#[inline(always)]
	pub fn lock_down_raw_network_and_other_input_and_output()
	{
		Capability::ensure_capabilities_dropped(&[Capability::NetRaw, Capability::RawIO]);
	}

	#[inline(always)]
	fn start_logging(&self)
	{
		self.logging_configuration.start_logging(self.running_interactively())
	}

	#[inline(always)]
	fn set_locale(&self)
	{
		let result = unsafe { setlocale(LC_ALL, self.locale.as_ptr() as *const _ as *const _) };
		assert!(!result.is_null(), "Could not set locale to `{:?}`", self.locale)
	}

	#[inline(always)]
	fn valid_hyper_threads_for_the_current_process(&self) -> BTreeSet<HyperThread>
	{
		HyperThread::valid_hyper_threads_for_the_current_process(self.proc_path())
	}

	#[inline(always)]
	fn set_current_process_affinity(valid_hyper_threads_for_the_current_process: &BTreeSet<HyperThread>) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		let cpu_set = CpuSet::from(valid_hyper_threads_for_the_current_process);
		cpu_set.set_current_process_affinity().map_err(|io_error| ProcessCommonConfigurationExecutionError::CouldNotSetCurrentProcessAffinity(io_error))
	}

	#[inline(always)]
	fn adjust_process_niceness(&self) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		self.process_niceness.adjust(self.proc_path())?;
		Ok(())
	}

	#[inline(always)]
	fn stop_logging(&self)
	{
		self.logging_configuration.stop_logging()
	}

	#[inline(always)]
	fn set_maximum_resource_limits(&self)
	{
		ResourceLimitsSet::defaultish(ResourceLimit::maximum_number_of_open_file_descriptors(self.proc_path()).expect("Could not read maximum number of file descriptors"));
	}

	#[inline(always)]
	fn write_system_control_values(&self) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		self.proc_path().write_system_control_values(&self.system_control_settings).map_err(|io_error| ProcessCommonConfigurationExecutionError::CouldNotWriteSystemControlValues(io_error))
	}

	#[inline(always)]
	fn rescan_all_pci_buses_and_devices(&self) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		self.sys_path().rescan_all_pci_buses_and_devices().map_err(|io_error| ProcessCommonConfigurationExecutionError::RescanOfAllPciBusesAndDevices(io_error))
	}

	#[inline(always)]
	fn validate_minimal_cpu_features(&self, uses_enhanced_intel_speedstep_technology: bool) -> Result<CpuFeatures, ProcessCommonConfigurationExecutionError>
	{
		CpuFeatures::validate_minimal_cpu_features(&self.warnings_to_suppress, uses_enhanced_intel_speedstep_technology).map_err(|explanation| ProcessCommonConfigurationExecutionError::CpuFeaturesValidationFailed(explanation))
	}

	#[inline(always)]
	fn validate_kernel_command_line(&self, isolated_cpus_required: bool, cpu_features: &CpuFeatures, additional_kernel_command_line_validations: impl FnOnce(&LinuxKernelCommandLineParameters) -> Result<(), String>) -> Result<BTreeSet<HyperThread>, ProcessCommonConfigurationExecutionError>
	{
		let linux_kernel_command_line_validator = LinuxKernelCommandLineValidator::new(self.proc_path());
		linux_kernel_command_line_validator.validate_and_find_isolated_hyper_threads(isolated_cpus_required, &self.warnings_to_suppress, cpu_features, additional_kernel_command_line_validations).map_err(|explanation| ProcessCommonConfigurationExecutionError::LinuxKernelCommandLineValidationFailed(explanation))
	}

	fn hyper_thread_sets(&self, isolated_hyper_threads_including_those_offline: BTreeSet<HyperThread>) -> (BTreeSet<HyperThread>, BTreeSet<HyperThread>, BTreeSet<HyperThread>, HyperThread)
	{
		#[inline(always)]
		fn find_master_logical_core(online_shared_hyper_threads: &BTreeSet<HyperThread>) -> HyperThread
		{
			let master_logical_core = HyperThread::last(online_shared_hyper_threads).unwrap();
			*master_logical_core
		}

		let valid_hyper_threads_for_the_current_process = HyperThread::valid_hyper_threads_for_the_current_process(self.proc_path());

		let (online_shared_hyper_threads_for_os, online_isolated_hyper_threads_for_os) = self.online_shared_and_isolated_hyper_threads(isolated_hyper_threads_including_those_offline);

		let online_shared_hyper_threads_for_process: BTreeSet<HyperThread> = online_shared_hyper_threads_for_os.difference(&valid_hyper_threads_for_the_current_process).cloned().collect();

		let online_isolated_hyper_threads_for_process: BTreeSet<HyperThread> = online_isolated_hyper_threads_for_os.difference(&valid_hyper_threads_for_the_current_process).cloned().collect();

		let master_logical_core = find_master_logical_core(&online_shared_hyper_threads_for_process);

		(online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core)
	}

	#[inline(always)]
	fn tell_linux_to_use_shared_hyper_threads_for_all_needs(&self, online_shared_hyper_threads: &BTreeSet<HyperThread>) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		use self::ProcessCommonConfigurationExecutionError::*;

		HyperThread::set_work_queue_hyper_thread_affinity(online_shared_hyper_threads, self.sys_path()).map_err(|io_error| CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads(io_error))?;

		HyperThread::force_watchdog_to_just_these_hyper_threads(online_shared_hyper_threads, self.proc_path()).map_err(|io_error| CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads(io_error))
	}

	#[inline(always)]
	fn daemonize_if_required(&mut self, main_loop: impl Fn(BTreeSet<HyperThread>, BTreeSet<HyperThread>, BTreeSet<HyperThread>, HyperThread) -> Result<Option<SignalNumber>, String>, online_shared_hyper_threads_for_os: BTreeSet<HyperThread>, online_shared_hyper_threads_for_process: BTreeSet<HyperThread>, online_isolated_hyper_threads_for_process: BTreeSet<HyperThread>, master_logical_core: HyperThread) -> Result<Option<SignalNumber>, String>
	{
		let main_loop = AssertUnwindSafe(|| main_loop(online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core));

		let success_or_failure = match self.daemonize.take()
		{
			None => catch_unwind(main_loop),

			Some(daemonize) =>
			{
				let daemonize_clean_up_on_exit = daemonize.daemonize(self.dev_path());

				let success_or_failure = catch_unwind(main_loop);

				daemonize_clean_up_on_exit.clean_up();

				success_or_failure
			}
		};

		match success_or_failure
		{
			Err(failure) =>
			{
				self.stop_logging();

				resume_unwind(failure)
			}

			Ok(reraise_signal_or_failure_explanation) => reraise_signal_or_failure_explanation,
		}
	}

	#[inline(always)]
	fn online_shared_and_isolated_hyper_threads(&self, isolated_hyper_threads_including_those_offline: BTreeSet<HyperThread>) -> (BTreeSet<HyperThread>, BTreeSet<HyperThread>)
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

	/// Are we running interactively?
	#[inline(always)]
	pub fn running_interactively(&self) -> bool
	{
		self.daemonize.is_none()
	}

	/// Disable transparent huge pages (THP).
	#[inline(always)]
	pub fn disable_transparent_huge_pages(&self) -> Result<(), ProcessCommonConfigurationExecutionError>
	{
		self.sys_path().disable_transparent_huge_pages()?;
		Ok(())
	}

	/// Verify hugetlbfs is supported.
	#[inline(always)]
	pub fn verify_hugetlbfs_is_supported(&self)
	{
		self.proc_path().filesystems().unwrap().verify_hugetlbfs_is_supported();
	}

	/// `/proc`
	#[inline(always)]
	pub fn proc_path(&self) -> &ProcPath
	{
		&self.proc_path
	}

	/// `/sys`
	#[inline(always)]
	pub fn sys_path(&self) -> &SysPath
	{
		&self.sys_path
	}

	/// `/dev`
	#[inline(always)]
	pub fn dev_path(&self) -> &DevPath
	{
		&self.dev_path
	}
}
