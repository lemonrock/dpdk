// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk as u64), including this file as u64), may be copied as u64), modified as u64), propagated as u64), or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Capability
{
	AuditControl = capability::CAP_AUDIT_CONTROL as u8,
	AuditRead = capability::CAP_AUDIT_READ as u8,
	AuditWrite = capability::CAP_AUDIT_WRITE as u8,
	
	BlockSuspend = capability::CAP_BLOCK_SUSPEND as u8,
	
	Chown = capability::CAP_CHOWN as u8, // Might want to retain this
	DiscretionaryAccessControlBypass = capability::CAP_DAC_OVERRIDE as u8,
	DiscretionaryAccessControlFileReadBypass = capability::CAP_DAC_READ_SEARCH as u8,
	FileOwnerBypass = capability::CAP_FOWNER as u8,
	FileSetId = capability::CAP_FSETID as u8,
	
	LockMemory = capability::CAP_IPC_LOCK as u8, // Probably want to retain this
	
	IpcOwner = capability::CAP_IPC_OWNER as u8,
	
	Kill = capability::CAP_KILL as u8,
	
	Lease = capability::CAP_LEASE as u8,
	
	Immutable = capability::CAP_LINUX_IMMUTABLE as u8,
	
	MandatoryAccessControlBypass = capability::CAP_MAC_ADMIN as u8,
	MandatoryAccessControlOverride = capability::CAP_MAC_OVERRIDE as u8,
	
	MakeNodes = capability::CAP_MKNOD as u8, // Might want to retain this
	
	SystemAdministration = capability::CAP_SYS_ADMIN as u8, // Might want to retain this
	NetworkAdministration = capability::CAP_NET_ADMIN as u8, // Might want to retain this
	BindPortsBelow1024 = capability::CAP_NET_BIND_SERVICE as u8, // Might want to retain this
	NetRaw = capability::CAP_NET_RAW as u8, // Probably want retain this for AF_PACKET
	
	SetUid = capability::CAP_SETUID as u8,
	SetGid = capability::CAP_SETGID as u8,
	
	SetFileCapabilities = capability::CAP_SETFCAP as u8,
	
	SetProcessCapabilities = capability::CAP_SETPCAP as u8,
	
	RebootAndKexecLoad = capability::CAP_SYS_BOOT as u8,
	
	Chroot = capability::CAP_SYS_CHROOT as u8,
	
	KernelModule = capability::CAP_SYS_MODULE as u8, // Might want to retain this
	
	Nice = capability::CAP_SYS_NICE as u8,
	
	ProcessAccounting = capability::CAP_SYS_PACCT as u8,
	
	PTrace = capability::CAP_SYS_PTRACE as u8,
	
	RawIO = capability::CAP_SYS_RAWIO as u8, // Might want to retain this
	
	Resource = capability::CAP_SYS_RESOURCE as u8,
	
	Time = capability::CAP_SYS_TIME as u8,
	
	TtyConfig = capability::CAP_SYS_TTY_CONFIG as u8,
	
	Syslog = capability::CAP_SYSLOG as u8,
	
	WakeAlarm = capability::CAP_WAKE_ALARM as u8, // Might want to retain this
}

impl Capability
{	
	pub fn clearAllAmbientCapabilities()
	{
		unsafe { prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_CLEAR_ALL, 0) };
	}
	
	pub fn ensureDropped(dropTheseCapabilitiesIfEnabled: &[Capability])
	{
		for capabilityToDrop in dropTheseCapabilitiesIfEnabled
		{
			if capabilityToDrop.processHas().unwrap_or(false)
			{
				capabilityToDrop.dropFromProcess().unwrap_or(());
			}
		}
	}
	
		
	pub fn processHas(&self) -> Option<bool>
	{
		match unsafe { prctl(PR_CAPBSET_READ, *self as c_ulong) }
		{
			1 => Some(true),
			
			0 => Some(false),
			
			-1 => match errno().0
			{
				E::EINVAL => None,
				
				illegal @ _ => panic!("Illegal error code '{}' from prctl()", illegal),
			},
			
			illegal @ _ => panic!("prctl() returned illegal result '{}'", illegal),
		}
	}
		
	pub fn dropFromProcess(&self) -> Result<(), ()>
	{
		match unsafe { prctl(PR_CAPBSET_DROP, *self as c_ulong) }
		{
			0 => Ok(()),
			
			-1 => match errno().0
			{
				E::EPERM => Err(()),
				E::EINVAL => panic!("Kernel does not support 'file' capabilities"),
				
				illegal @ _ => panic!("Illegal error code '{}' from prctl()", illegal),
			},
			
			illegal @ _ => panic!("prctl() returned illegal result '{}'", illegal),
		}
	}
}
