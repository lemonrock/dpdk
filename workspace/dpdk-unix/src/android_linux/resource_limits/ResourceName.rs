// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Name for a resource which has a soft and a hard limit.
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceName
{
	MaximumSizeOfVirtualMemoryAddressSpaceInBytes = ::libc::RLIMIT_AS,
	MaximumSizeOfACoreDumpFileInBytes = ::libc::RLIMIT_CORE,
	CpuTimeLimitInSeconds = ::libc::RLIMIT_CPU,
	MaximumSizeOfProcessDataSegmentInBytes = ::libc::RLIMIT_DATA,
	MaximumSizeOfProcessResidentSetSizeInBytes = ::libc::RLIMIT_RSS,
	MaximumSizeOfProcessStackInBytes = ::libc::RLIMIT_STACK,
	MaximumSizeOfFilesThatProcessCanCreateInBytes = ::libc::RLIMIT_FSIZE,
	MaximumNumberOfBytesThatProcessCanMemLock = ::libc::RLIMIT_MEMLOCK,
	MaximumNumberOfBytesForPosixMessageQueues = ::libc::RLIMIT_MSGQUEUE,
	NiceCeilingLargerIsBetter = ::libc::RLIMIT_NICE,
	RealTimePriorityCeilingLargerIsBetter = ::libc::RLIMIT_RTPRIO,
	MaximumNumberOfFileDescriptors = ::libc::RLIMIT_NOFILE,
	MaximumNumberOfProcessAndThreads = ::libc::RLIMIT_NPROC,
	RealTimePriorityLimitInMicroseconds = ::libc::RLIMIT_RTTIME,
	MaximumNumberOfSignalsPending = ::libc::RLIMIT_SIGPENDING,
}

impl ResourceName
{
	/// Sets a resource to a hard and soft limit.
	///
	/// Panics if it can not be set.
	#[inline(always)]
	pub fn set(&self, soft_and_hard_resource_limit: &SoftAndHardResourceLimit)
	{
		soft_and_hard_resource_limit.set(*self as i32)
	}
	
	/// Gets a resources hard and soft limits.
	///
	/// Panics if they can not be obtained.
	#[inline(always)]
	pub fn get(&self) -> SoftAndHardResourceLimit
	{
		SoftAndHardResourceLimit::get(*self as i32)
	}
}
