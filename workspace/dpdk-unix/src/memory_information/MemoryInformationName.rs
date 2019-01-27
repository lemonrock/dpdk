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
	
	Unknown(Box<[u8]>),
}

impl MemoryInformationName
{
	//noinspection SpellCheckingInspection
	/// Parse a memory statistic name.
	///
	/// This list is NOT definitive; names come and go.
	#[inline(always)]
	pub(crate) fn parse(value: &[u8], memory_information_name_prefix: &[u8]) -> MemoryInformationName
	{
		use self::MemoryInformationName::*;

		if !value.starts_with(memory_information_name_prefix)
		{
			return Unknown(value.to_vec().into_boxed_slice())
		}

		match &value[memory_information_name_prefix.len() .. ]
		{
			b"MemTotal" => TotalPhysicalRam,
			b"MemFree" => FreePhysicalRam,
			b"MemAvailable" => AvailablePhysicalRam,
			b"Buffers" => UsedAsFileBuffersPhysicalRam,
			b"Cached" => UsedAsCachePhysicalRam,
			
			b"SwapTotal" => TotalSwap,
			b"SwapFree" => FreeSwap,
			b"SwapCached" => UsedAsCacheSwap,
			
			b"Active" => ActiveFileBufferAndCacheInUse,
			b"Inactive" => InactiveFileBufferAndCacheAvailable,
			b"Active(anon" => AnonymousActive,
			b"Inactive(anon" => AnonymousInactive,
			b"Active(file" => FileActive,
			b"Inactive(file" => FileInactive,
			
			b"Unevictable" => Unevictable,
			
			b"Dirty" => WaitingToBeWrittenBackToDisks,
			b"Writeback" => CurrentlyBeingWrittenBackToDisks,
			b"Bounce" => UsedForBlockDeviceBounceBuffers,
			b"NFS_Unstable" => NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,
			b"WritebackTmp" => MemoryUsedByFuseForTemporaryWritebackBuffers,
			
			b"AnonPages" => AnonymousMemoryMappedUsingMmap,
			b"Mapped" => FilesMappedUsingMmap,
			b"Shmem" => Shmem,
			b"Mlocked" => LockedByMlock,
			
			b"Slab" => Slab,
			b"SReclaimable" => SlabReclaimable,
			b"SUnreclaim" => SlabUnreclaimable,
			
			b"KernelStack" => KernelStack,
			b"PageTables" => MemoryDedicatedToLowestPageTableLevel,
			b"CommitLimit" => CommitLimit,
			b"Committed_AS" => WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
			
			b"VmallocTotal" => TotalVirtualAddressSpaceEgByMalloc,
			b"VmallocUsed" => UsedVirtualAddressSpaceEgByMalloc,
			b"VmallocChunk" => LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,
			
			b"HugePages_Total" => TotalNumberOfHugePages,
			b"HugePages_Free" => FreeNumberOfHugePages,
			b"HugePages_Rsvd" => ReservedNumberOfHugePages,
			b"HugePages_Surp" => SurplusNumberOfHugePages,
			b"Hugepagesize" => SizeOfAHugePage,
			b"AnonHugePages" => TransparentHugePagesMemoryUsage,
			b"DirectMap4k" => DirectMap4k,
			b"DirectMap2M" => DirectMap2M,
			
			b"HardwareCorrupted" => HardwareCorrupted,
			b"HighTotal" => TotalHighNotDirectlyMappedIntoKernelSpace,
			b"HighFree" => FreeHighNotDirectlyMappedIntoKernelSpace,
			b"LowTotal" => TotalLowDirectlyMappedIntoKernelSpace,
			b"LowFree" => FreeLowDirectlyMappedIntoKernelSpace,
			b"ShmemHugePages" => ShmemHugePageUsage,
			b"ShmemPmdMapped" => ShmemMemoryMappedIntoUserSpaceUsingHugePages,
			
			name @ _ => Unknown(name.to_vec().into_boxed_slice()),
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
