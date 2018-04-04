// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk as u32), including this file as u32), may be copied as u32), modified as u32), propagated as u32), or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	// Capabilities commented out can not be represented in the legacy API because they bit shift to values greater than can be represented by a u32
	
	pub flags CapabilitySet: u32
	{
		const AuditControl = 1 << (capability::CAP_AUDIT_CONTROL as u32),
		//const AuditRead = 1 << (capability::CAP_AUDIT_READ as u32),
		const AuditWrite = 1 << (capability::CAP_AUDIT_WRITE as u32),
		
		//const BlockSuspend = 1 << (capability::CAP_BLOCK_SUSPEND as u32),
		
		const Chown = 1 << (capability::CAP_CHOWN as u32),
		const DiscretionaryAccessControlBypass = 1 << (capability::CAP_DAC_OVERRIDE as u32),
		const DiscretionaryAccessControlFileReadBypass = 1 << (capability::CAP_DAC_READ_SEARCH as u32),
		const FileOwnerBypass = 1 << (capability::CAP_FOWNER as u32),
		const FileSetId = 1 << (capability::CAP_FSETID as u32),
		
		const LockMemory = 1 << (capability::CAP_IPC_LOCK as u32), // want
		
		const IpcOwner = 1 << (capability::CAP_IPC_OWNER as u32),
		
		const Kill = 1 << (capability::CAP_KILL as u32),
		
		const Lease = 1 << (capability::CAP_LEASE as u32),
		
		const Immutable = 1 << (capability::CAP_LINUX_IMMUTABLE as u32),
		
		//const MandatoryAccessControlBypass = 1 << (capability::CAP_MAC_ADMIN as u32),
		//const MandatoryAccessControlOverride = 1 << (capability::CAP_MAC_OVERRIDE as u32),
		
		const MakeNodes = 1 << (capability::CAP_MKNOD as u32),
		
		const SystemAdministration = 1 << (capability::CAP_SYS_ADMIN as u32),
		const NetworkAdministration = 1 << (capability::CAP_NET_ADMIN as u32), // Might want to retain this
		const BindPortsBelow1024 = 1 << (capability::CAP_NET_BIND_SERVICE as u32), // Probably want to retain this
		const NetRaw = 1 << (capability::CAP_NET_RAW as u32), // Want to retain this for AF_PACKET
		
		const SetUid = 1 << (capability::CAP_SETUID as u32),
		const SetGid = 1 << (capability::CAP_SETGID as u32),
		
		const SetFileCapabilities = 1 << (capability::CAP_SETFCAP as u32),
		
		const SetProcessCapabilities = 1 << (capability::CAP_SETPCAP as u32),
		
		const RebootAndKexecLoad = 1 << (capability::CAP_SYS_BOOT as u32),
		
		const Chroot = 1 << (capability::CAP_SYS_CHROOT as u32),
		
		const KernelModule = 1 << (capability::CAP_SYS_MODULE as u32),
		
		const Nice = 1 << (capability::CAP_SYS_NICE as u32),
		
		const ProcessAccounting = 1 << (capability::CAP_SYS_PACCT as u32),
		
		const PTrace = 1 << (capability::CAP_SYS_PTRACE as u32),
		
		const RawIO = 1 << (capability::CAP_SYS_RAWIO as u32), // Might want to retain this
		
		const Resource = 1 << (capability::CAP_SYS_RESOURCE as u32),
		
		const Time = 1 << (capability::CAP_SYS_TIME as u32),
		
		const TtyConfig = 1 << (capability::CAP_SYS_TTY_CONFIG as u32),
		
		//const Syslog = 1 << (capability::CAP_SYSLOG as u32),
		
		//const WakeAlarm = 1 << (capability::CAP_WAKE_ALARM as u32), // Might want to retain this
	}
}

impl Default for CapabilitySet
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl CapabilitySet
{
	fn newVersion3ForCurrentProcess() -> __user_cap_header_struct
	{
		let pid = unsafe { getpid() };
		
		__user_cap_header_struct
		{
			version: _LINUX_CAPABILITY_VERSION_3,
			pid: pid,
		}
	}
	
	// Permitted, Inheritable, Effective
	pub fn get() -> (CapabilitySet, CapabilitySet, CapabilitySet)
	{
		let mut header = Self::newVersion3ForCurrentProcess();
		let mut data = unsafe { zeroed() };
		
		match Self::capget(&mut header, &mut data)
		{
			0 => (),
			
			-1 => match errno().0
			{
				E::EFAULT => panic!("capget() is not using Version 3 of the Kernel API"), // We can probe by setting &mut data as NULL, but why bother? The last revision was in 2008 in Linux 2.6.26...
				
				E::EINVAL => panic!("Invalid value in capget() syscall"),
				E::EPERM => panic!("Permission denied should not occur"),
				E::ESRCH => panic!("Wow - out process id does not exist"),
				
				illegal @ _ => panic!("capget() syscall returned illegal error code '{}'", illegal),
			},
				
			illegal @ _ => panic!("capget() syscall returned illegal result '{}'", illegal),
		}
		
		(
			Self::from_bits_truncate(data.permitted),
			Self::from_bits_truncate(data.inheritable),
			Self::from_bits_truncate(data.effective),
		)
	}
	
	pub fn set(permitted: CapabilitySet, inheritable: CapabilitySet, effective: CapabilitySet)
	{
		let mut header = Self::newVersion3ForCurrentProcess();
		let mut data = __user_cap_data_struct
		{
			effective: effective.bits(),
			permitted: permitted.bits(),
			inheritable: inheritable.bits(),
		};
		
		match Self::capset(&mut header, &mut data)
		{
			0 => (),
			
			-1 => match errno().0
			{
				E::EFAULT => panic!("capset() is not using Version 3 of the Kernel API"),
				
				E::EINVAL => panic!("Invalid value in capset() syscall"),
				E::EPERM => panic!("Permission denied should not occur"),
				E::ESRCH => panic!("Wow - out process id does not exist"),
				
				illegal @ _ => panic!("capset() syscall returned illegal error code '{}'", illegal),
			},
				
			illegal @ _ => panic!("capset() syscall returned illegal result '{}'", illegal),
		}
	}
	
	fn capget(hdrp: cap_user_header_t, datap: cap_user_data_t) -> c_long
	{
		unsafe { syscall(SYS_capget as c_long, hdrp as *mut c_void, datap as *mut c_void) }
	}
	
	fn capset(hdrp: cap_user_header_t, datap: cap_user_data_t) -> c_long
	{	
		unsafe { syscall(SYS_capset as c_long, hdrp as *mut c_void, datap as *mut c_void) }
	}
}
