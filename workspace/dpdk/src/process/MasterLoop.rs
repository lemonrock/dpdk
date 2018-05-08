// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Whilst kernel modules may be loaded by this process, we do not unload them on process exit.
///
/// This is because at this stage we typically won't have permissions to do so.
pub struct MasterLoop
{
	#[serde(default = "")] dev_path: PathBuf,
	proc_path: PathBuf,
	dpdk_provided_kernel_modules_path: PathBuf,
	
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

// TODO: Load / unload huge pages.
// TODO: Load / unload pci device changes.
// TODO: Detect need to use pci devices.
// TODO: initialize DPDK
// TODO: Initialize slave logical cores (and service cores).
// TODO: Set termination signal for all logical cores.
// TODO: Wait for all logical cores
// TODO: Stop all ethernet devices.

impl MasterLoop
{
	/// Executes.
	///
	/// Panics may be caught but are re-raised.
	///
	/// If running interactively (`daemonize == None`), then `SIGINT` and `SIGQUIT` are intercepted and will be re-raised if caught so that any parent shell can behave correctly.
	#[inline(always)]
	pub fn execute(&self, path_configuration: &PathConfiguration, daemonize: Option<Daemonize>)
	{
		// essential_kernel_modules are drom PciKernelDriver & if we're going to use rte_kni.
		
		let reraise_signal = if let Some(daemonize) = daemonize
		{
			let daemonize_clean_up_on_exit = daemonize.daemonize();
			let success_or_failure = catch_unwind(|| self.execute_after_daemonizing(path_configuration, essential_kernel_modules, false));
			daemonize_clean_up_on_exit.clean_up();
			
			match success_or_failure
			{
				Err(failure) => resume_unwind(failure),
				Ok(reraise_signal) => reraise_signal,
			}
		}
		else
		{
			self.execute_after_daemonizing(path_configuration, essential_kernel_modules, true)
		};
		
		if let Some(reraise_signal_number) = reraise_signal
		{
			unsafe { raise(reraise_signal_number) };
		}
	}
	
	#[inline(always)]
	fn execute_after_daemonizing(&self, path_configuration: &PathConfiguration, essential_kernel_modules: &[EssentialKernelModule], running_interactively: bool) -> Option<SignalNumber>
	{
		self.set_maximum_resource_limits();
		
		// TODO: PciNetDevicesConfiguration
		let pci_devices_and_original_driver_names = PciNetDevicesConfiguration.take_for_use_with_dpdk(&path_configuration.sys_path);
		
		// TODO: After this, we need to free all pci_devices_and_original_driver_names on failure / normal exit.
		
		// TODO: pci_devices_and_original_driver_names => essential_kernel_modules
		
		// TODO: If using the RTE KNI virtual devicem need to load rte_kni
		
		self.load_kernel_modules(path_configuration, essential_kernel_modules);
		
		Self::block_all_signals_before_initializing_dpdk_so_that_slave_logical_cores_do_not_handle_signals();
		
		// TODO: init DPDK
		
		let success_or_failure = catch_unwind(|| self.execute_after_dpdk_initialized(running_interactively));
		
		Self::dpdk_clean_up();
		
		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),
			Ok(reraise_signal) => reraise_signal,
		}
	}
	
	#[inline(always)]
	fn execute_after_dpdk_initialized(&self, running_interactively: bool) -> Option<SignalNumber>
	{
		Self::remove_nearly_all_capabilities();
		
		Self::disable_dumpable();
		
		Self::no_new_privileges();
		
		Self::clear_all_ambient_capabilities();
		
		Self::lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();
		
		// TODO: Initialize logical slave cores and services..
		
		let reraise_signal = Self::infinite_signal_handling_loop(running_interactively);
		
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
	fn load_kernel_modules(&self, path_configuration: &PathConfiguration, essential_kernel_modules: &[EssentialKernelModule])
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			let mut modules_loaded = LinuxKernelModulesList::parse_currently_loaded_linux_kernel_modules_list(&path_configuration.proc_path);
			let mut essential_kernel_modules_to_unload = EssentialKernelModulesToUnload::new();
			for essential_kernel_module in essential_kernel_modules.iter()
			{
				essential_kernel_module.load_if_necesary(modules_loaded, &path_configuration.dpdk_provided_kernel_modules_path, &mut essential_kernel_modules_to_unload, &path_configuration: &PathConfiguration.dev_path);
				essential_kernel_module.load_if_necesary();
			}
		}
	}
	
	#[inline(always)]
	fn block_all_signals_before_initializing_dpdk_so_that_slave_logical_cores_do_not_handle_signals()
	{
		block_all_signals_on_current_thread();
	}
	
	#[inline(always)]
	fn infinite_signal_handling_loop(running_interactively: bool) -> Option<SignalNumber>
	{
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
