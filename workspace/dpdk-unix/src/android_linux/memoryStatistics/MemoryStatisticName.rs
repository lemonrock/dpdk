// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Some old help here: https://www.centos.org/docs/5/html/5.1/Deployment_Guide/s2-proc-meminfo.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryStatisticName
{
	TotalPhyiscalRam,
	FreePhysicalRam,
	AvailablePhysicalRam, // estimate of physical ram available for starting new applications. Always larger than FreePhysicalRam; see https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/commit/?id=34e431b0ae398fc54ea69ff85ec700722c9da773
	UsedAsFileBuffersPhysicalRam,
	UsedAsCachePhysicalRam,
	
	TotalSwap,
	FreeSwap,
	UsedAsCacheSwap,
	
	ActiveFileBufferAndCacheInUse,
	InactiveFileBufferAndCacheAvailable,
	AnonymousActive,
	AnonymousInactive,
	FileActive,
	FileInactive,
	
	Unevictable,
	
	WaitingToBeWrittenBackToDisks, // aka 'Dirty'
	CurrentlyBeingWrittenBackToDisks, // aka 'Writeback'
	UsedForBlockDeviceBounceBuffers,
	NetworkFileSystemUnstablePagesSentToServerButNotYetCommitedToStableStorage,
	MemoryUsedByFuseForTemporaryWritebackBuffers,
	
	AnononymousMemoryMappedUsingMmap,
	FilesMappedUsingMmap,
	Shmem,
	LockedByMlock,
	
	Slab,
	SlabReclaimable,
	SlabUnreclaimable,
	
	KernelStack,
	MemoryDedicatedToLowestPageTableLevel,
	CommitLimit,
	WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
	
	TotalVirtualAddressSpaceEgByMalloc,
	UsedVirtualAddressSpaceEgByMalloc,
	LarestContiguousChunkInVirtualAddressSpaceEgByMalloc,
	
	TotalNumberOfHugePages, // The number is derived by dividing SizeOfAHugePage by the megabytes set aside for hugepages specified in /proc/sys/vm/hugetlb_pool
	FreeNumberOfHugePages,
	ReservedNumberOfHugePages,
	SurplusNumberOfHugesPages,
	SizeOfAHugePage,
	TransparentHugePagesMemoryUsage, // Not mapped using tlbfs
	DirectMap4k,
	DirectMap2M,

	HardwareCorrupted,
	TotalHighNotDirectlyMappedIntoKernelSpace,
	FreeHighNotDirectlyMappedIntoKernelSpace,
	TotalLowDirectlyMappedIntoKernelSpace,
	FreeLowDirectlyMappedIntoKernelSpace,
	ShmemHugePageUsage,
	ShmemMemoryMappedIntoUserSpaceUsingHugePages,
	
	Unknown(String),
}

impl MemoryStatisticName
{
	// This list is NOT definitive; names come and go
	#[inline(always)]
	pub fn parse(value: &str, memoryStatisticNamePrefix: &str) -> MemoryStatisticName
	{
		if !value.starts_with(memoryStatisticNamePrefix)
		{
			return MemoryStatisticName::Unknown(value.to_owned());
		}
		
		match &value[memoryStatisticNamePrefix.len()..]
		{
			"MemTotal" => MemoryStatisticName::TotalPhyiscalRam,
			"MemFree" => MemoryStatisticName::FreePhysicalRam,
			"MemAvailable" => MemoryStatisticName::AvailablePhysicalRam,
			"Buffers" => MemoryStatisticName::UsedAsFileBuffersPhysicalRam,
			"Cached" => MemoryStatisticName::UsedAsCachePhysicalRam,
			
			"SwapTotal" => MemoryStatisticName::TotalSwap,
			"SwapFree" => MemoryStatisticName::FreeSwap,
			"SwapCached" => MemoryStatisticName::UsedAsCacheSwap,
			
			"Active" => MemoryStatisticName::ActiveFileBufferAndCacheInUse,
			"Inactive" => MemoryStatisticName::InactiveFileBufferAndCacheAvailable,
			"Active(anon" => MemoryStatisticName::AnonymousActive,
			"Inactive(anon" => MemoryStatisticName::AnonymousInactive,
			"Active(file" => MemoryStatisticName::FileActive,
			"Inactive(file" => MemoryStatisticName::FileInactive,
			
			"Unevictable" => MemoryStatisticName::Unevictable,
			
			"Dirty" => MemoryStatisticName::WaitingToBeWrittenBackToDisks,
			"Writeback" => MemoryStatisticName::CurrentlyBeingWrittenBackToDisks,
			"Bounce" => MemoryStatisticName::UsedForBlockDeviceBounceBuffers,
			"NFS_Unstable" => MemoryStatisticName::NetworkFileSystemUnstablePagesSentToServerButNotYetCommitedToStableStorage,
			"WritebackTmp" => MemoryStatisticName::MemoryUsedByFuseForTemporaryWritebackBuffers,
			
			"AnonPages" => MemoryStatisticName::AnononymousMemoryMappedUsingMmap,
			"Mapped" => MemoryStatisticName::FilesMappedUsingMmap,
			"Shmem" => MemoryStatisticName::Shmem,
			"Mlocked" => MemoryStatisticName::LockedByMlock,
			
			"Slab" => MemoryStatisticName::Slab,
			"SReclaimable" => MemoryStatisticName::SlabReclaimable,
			"SUnreclaim" => MemoryStatisticName::SlabUnreclaimable,
			
			"KernelStack" => MemoryStatisticName::KernelStack,
			"PageTables" => MemoryStatisticName::MemoryDedicatedToLowestPageTableLevel,
			"CommitLimit" => MemoryStatisticName::CommitLimit,
			"Committed_AS" => MemoryStatisticName::WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
			
			"VmallocTotal" => MemoryStatisticName::TotalVirtualAddressSpaceEgByMalloc,
			"VmallocUsed" => MemoryStatisticName::UsedVirtualAddressSpaceEgByMalloc,
			"VmallocChunk" => MemoryStatisticName::LarestContiguousChunkInVirtualAddressSpaceEgByMalloc,
			
			"HugePages_Total" => MemoryStatisticName::TotalNumberOfHugePages,
			"HugePages_Free" => MemoryStatisticName::FreeNumberOfHugePages,
			"HugePages_Rsvd" => MemoryStatisticName::ReservedNumberOfHugePages,
			"HugePages_Surp" => MemoryStatisticName::SurplusNumberOfHugesPages,
			"Hugepagesize" => MemoryStatisticName::SizeOfAHugePage,
			"AnonHugePages" => MemoryStatisticName::TransparentHugePagesMemoryUsage,
			"DirectMap4k" => MemoryStatisticName::DirectMap4k,
			"DirectMap2M" => MemoryStatisticName::DirectMap2M,
			
			"HardwareCorrupted" => MemoryStatisticName::HardwareCorrupted,
			"HighTotal" => MemoryStatisticName::TotalHighNotDirectlyMappedIntoKernelSpace,
			"HighFree" => MemoryStatisticName::FreeHighNotDirectlyMappedIntoKernelSpace,
			"LowTotal" => MemoryStatisticName::TotalLowDirectlyMappedIntoKernelSpace,
			"LowFree" => MemoryStatisticName::FreeLowDirectlyMappedIntoKernelSpace,
			"ShmemHugePages" => MemoryStatisticName::ShmemHugePageUsage,
			"ShmemPmdMapped" => MemoryStatisticName::ShmemMemoryMappedIntoUserSpaceUsingHugePages,
			
			name @ _ => MemoryStatisticName::Unknown(name.to_owned()),
		}
	}
	
	#[inline(always)]
	pub fn isDeprecatedOrNotUnderstood(&self) -> bool
	{
		match *self
		{
			MemoryStatisticName::HardwareCorrupted => true,
			MemoryStatisticName::TotalHighNotDirectlyMappedIntoKernelSpace => true,
			MemoryStatisticName::FreeHighNotDirectlyMappedIntoKernelSpace => true,
			MemoryStatisticName::TotalLowDirectlyMappedIntoKernelSpace => true,
			MemoryStatisticName::FreeLowDirectlyMappedIntoKernelSpace => true,
			MemoryStatisticName::ShmemHugePageUsage => true,
			MemoryStatisticName::ShmemMemoryMappedIntoUserSpaceUsingHugePages => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	pub fn unit(&self) -> MemoryStatisticUnit
	{
		match *self
		{
			MemoryStatisticName::TotalNumberOfHugePages => MemoryStatisticUnit::Count,
			MemoryStatisticName::FreeNumberOfHugePages => MemoryStatisticUnit::Count,
			MemoryStatisticName::ReservedNumberOfHugePages => MemoryStatisticUnit::Count,
			MemoryStatisticName::SurplusNumberOfHugesPages => MemoryStatisticUnit::Count,
			
			_ => MemoryStatisticUnit::KiloByte,
		}
	}
}
