// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Memory information names for a process.
///
/// Some old help here: <https://www.centos.org/docs/5/html/5.1/Deployment_Guide/s2-proc-meminfo.html>.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryInformationName
{
	TotalPhysicalRam,
	FreePhysicalRam,
	/// An estimate of physical ram available for starting new applications.
	/// Always larger than `FreePhysicalRam`; see <https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/commit/?id=34e431b0ae398fc54ea69ff85ec700722c9da773>.
	AvailablePhysicalRam,
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
	
	/// aka 'Dirty'.
	WaitingToBeWrittenBackToDisks,
	/// aka 'Writeback'.
	CurrentlyBeingWrittenBackToDisks,
	UsedForBlockDeviceBounceBuffers,
	NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,
	MemoryUsedByFuseForTemporaryWritebackBuffers,
	
	AnonymousMemoryMappedUsingMmap,
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
	LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,
	
	/// The number is derived by dividing `SizeOfAHugePage` by the megabytes set aside for `hugepages` specified in `/proc/sys/vm/hugetlb_pool`.
	TotalNumberOfHugePages,
	FreeNumberOfHugePages,
	ReservedNumberOfHugePages,
	SurplusNumberOfHugePages,
	SizeOfAHugePage,
	/// Not mapped using `tlbfs`.
	TransparentHugePagesMemoryUsage,
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

impl MemoryInformationName
{
	//noinspection SpellCheckingInspection
	/// Parse a memory statistic name.
	///
	/// This list is NOT definitive; names come and go.
	#[inline(always)]
	pub(crate) fn parse(value: &str, memory_information_name_prefix: &str) -> MemoryInformationName
	{
		use self::MemoryInformationName::*;
		
		if !value.starts_with(memory_information_name_prefix)
		{
			return Unknown(value.to_owned());
		}
		
		match &value[memory_information_name_prefix.len()..]
		{
			"MemTotal" => TotalPhysicalRam,
			"MemFree" => FreePhysicalRam,
			"MemAvailable" => AvailablePhysicalRam,
			"Buffers" => UsedAsFileBuffersPhysicalRam,
			"Cached" => UsedAsCachePhysicalRam,
			
			"SwapTotal" => TotalSwap,
			"SwapFree" => FreeSwap,
			"SwapCached" => UsedAsCacheSwap,
			
			"Active" => ActiveFileBufferAndCacheInUse,
			"Inactive" => InactiveFileBufferAndCacheAvailable,
			"Active(anon" => AnonymousActive,
			"Inactive(anon" => AnonymousInactive,
			"Active(file" => FileActive,
			"Inactive(file" => FileInactive,
			
			"Unevictable" => Unevictable,
			
			"Dirty" => WaitingToBeWrittenBackToDisks,
			"Writeback" => CurrentlyBeingWrittenBackToDisks,
			"Bounce" => UsedForBlockDeviceBounceBuffers,
			"NFS_Unstable" => NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,
			"WritebackTmp" => MemoryUsedByFuseForTemporaryWritebackBuffers,
			
			"AnonPages" => AnonymousMemoryMappedUsingMmap,
			"Mapped" => FilesMappedUsingMmap,
			"Shmem" => Shmem,
			"Mlocked" => LockedByMlock,
			
			"Slab" => Slab,
			"SReclaimable" => SlabReclaimable,
			"SUnreclaim" => SlabUnreclaimable,
			
			"KernelStack" => KernelStack,
			"PageTables" => MemoryDedicatedToLowestPageTableLevel,
			"CommitLimit" => CommitLimit,
			"Committed_AS" => WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
			
			"VmallocTotal" => TotalVirtualAddressSpaceEgByMalloc,
			"VmallocUsed" => UsedVirtualAddressSpaceEgByMalloc,
			"VmallocChunk" => LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,
			
			"HugePages_Total" => TotalNumberOfHugePages,
			"HugePages_Free" => FreeNumberOfHugePages,
			"HugePages_Rsvd" => ReservedNumberOfHugePages,
			"HugePages_Surp" => SurplusNumberOfHugePages,
			"Hugepagesize" => SizeOfAHugePage,
			"AnonHugePages" => TransparentHugePagesMemoryUsage,
			"DirectMap4k" => DirectMap4k,
			"DirectMap2M" => DirectMap2M,
			
			"HardwareCorrupted" => HardwareCorrupted,
			"HighTotal" => TotalHighNotDirectlyMappedIntoKernelSpace,
			"HighFree" => FreeHighNotDirectlyMappedIntoKernelSpace,
			"LowTotal" => TotalLowDirectlyMappedIntoKernelSpace,
			"LowFree" => FreeLowDirectlyMappedIntoKernelSpace,
			"ShmemHugePages" => ShmemHugePageUsage,
			"ShmemPmdMapped" => ShmemMemoryMappedIntoUserSpaceUsingHugePages,
			
			name @ _ => Unknown(name.to_owned()),
		}
	}
	
	/// Deprecated or not understood memory statistic names.
	#[inline(always)]
	pub fn is_deprecated_or_not_understood(&self) -> bool
	{
		use self::MemoryInformationName::*;
		
		match *self
		{
			HardwareCorrupted => true,
			TotalHighNotDirectlyMappedIntoKernelSpace => true,
			FreeHighNotDirectlyMappedIntoKernelSpace => true,
			TotalLowDirectlyMappedIntoKernelSpace => true,
			FreeLowDirectlyMappedIntoKernelSpace => true,
			ShmemHugePageUsage => true,
			ShmemMemoryMappedIntoUserSpaceUsingHugePages => true,
			
			_ => false,
		}
	}
	
	/// Associated memory statistic unit.
	#[inline(always)]
	pub fn unit(&self) -> MemoryInformationUnit
	{
		use self::MemoryInformationName::*;
		use self::MemoryInformationUnit::*;
		
		match *self
		{
			TotalNumberOfHugePages => Count,
			FreeNumberOfHugePages => Count,
			ReservedNumberOfHugePages => Count,
			SurplusNumberOfHugePages => Count,
			
			_ => KiloByte,
		}
	}
}
