// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	// Flags commented '// ' are special
	
	/// Mount flags.
	#[allow(missing_docs)]
	#[derive(Serialize, Deserialize)]
	pub struct MountFlags: u64
	{
		// const BIND = ::libc::MS_BIND;
		
		///
		const DirectoryChangesAreSynchronous = ::libc::MS_DIRSYNC;
		
		///
		const PermitMandatoryLocking = ::libc::MS_MANDLOCK;
		
		// const MOVE = ::libc::MS_MOVE;
		
		///
		const DoNotUpdateAccessTimes = ::libc::MS_NOATIME;
		
		///
		const DoNotAllowDeviceFiles = ::libc::MS_NODEV;
		
		/// Implicit if `DoNotUpdateAccessTimes` is specified.
		const DoNotUpdateDirectoryAccessTimes = ::libc::MS_NODIRATIME;
		
		///
		const DoNotAllowProgramsToBeExecuted = ::libc::MS_NOEXEC;
		
		///
		const DoNotHonourSetUidAndSetGidPermissions = ::libc::MS_NOSUID;
		
		// const PRIVATE = ::libc::MS_PRIVATE;
		
		// const RecursiveBindMount = ::libc::MS_REC;
		
		///
		const RelaxedAccessTimeUpdates = ::libc::MS_RELATIME;
		
		// const REMOUNT = ::libc::MS_REMOUNT;
		
		// const SHARED = ::libc::MS_SHARED;
		
		///
		const SilenceSomeKernelWarningMessages = ::libc::MS_SILENT;
		
		// const SLAVE = ::libc::MS_SLAVE;
		
	 	/// Overrides `DoNotUpdateAccessTimes` and `DoNotUpdateDirectoryAccessTimes`.
		const AlwaysUpdateTheLastAccessTime = ::libc::MS_STRICTATIME;
		
		///
		const FileWritesAreSynchronous = ::libc::MS_SYNCHRONOUS;
		
		// const UNBINDABLE = ::libc::MS_UNBINDABLE;
		
		// MS_RDONLY
		// MS_LAZYTIME
	}
}
