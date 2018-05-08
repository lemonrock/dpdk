// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct MasterLoop
{
	daemonize: Daemonize,
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

impl MasterLoop
{
	#[inline(always)]
	pub fn execute(&self)
	{
		Self::remove_nearly_all_capabilities();
		
		Self::disable_dumpable();
		
		Self::no_new_privileges();
		
		block_all_signals_on_current_thread_bar_hang_up_and_terminate_and_child();
		
		Self::start_a_new_process_group();
		
		let daemonize_clean_up_on_exit = self.daemonize.daemonize();
		
		block_all_signals_on_current_thread_bar_child();
		
		// signal_handling
		
		// init DPDK
		
		
		
		
		
		
		
		// sigaction();
		
		#[inline(always)]
		fn signal_handling()
		{
			
			unsafe extern "C" fn sigint(signal_number: i32)
			{
			
			}
			
		}
		
		while self.should_continue()
		{
			self.execute_once()
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
				//KernelModule,
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
	fn start_a_new_process_group()
	{
		let result = unsafe { setpgid(0, 0) };
		if likely(result == 0)
		{
			return;
		}
		match result
		{
			-1 => panic!("Could not setpgid"),
			_ => panic!("Positive value from setpgid"),
		}
	}
	
	#[inline(always)]
	fn daemonize(&self)
	{
	
	
	}
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	#[inline(always)]
	fn should_continue(&self) -> bool
	{
		self.should_function_terminate.should_continue()
	}
}
