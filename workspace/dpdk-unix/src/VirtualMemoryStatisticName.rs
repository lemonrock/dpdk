// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A list of known virtual memory statistics related to NUMA nodes.
///
/// There are far more statistics than those listed here.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualMemoryStatisticName
{
	/// Found in `/sys/devices/system/node/node<X>/vmstat` where `<X>` is a zero-based NUMA node number.
	NumberOFreePages,
	NumberOfBatchAllocatedPages,
	NumberOfInactiveAnonymousPages,
	NumberOfActiveAnonymousPages,
	NumberOfInactiveFilePages,
	NumberOfActiveFilePages,
	NumberOfUnevictablePages,
	NumberOfLockedPages,
	NumberOfAnonymousPages,
	NumberOfMappedPages,
	NumberOfFilePages,
	NumberOfDirtyPages,
	NumberOfWritebackPages,
	NumberOfReclaimableSlabPages,
	NumberOfUnreclaimableSlabPages,
	NumberOfPageTablePages,
	NumberOfKernelStackPages,
	NumberOfUnstablePages,
	NumberOfBouncePages,
	NumberOfVirtualMemoryWritePages,
	NumberOfVirtualMemoryImmediateReclaimPages,
	NumberOfWritebackTemporaryPages,
	NumberOfIsolatedAnonymousPages,
	NumberOfIsolatedFilePages,
	NumberOfShmemPages,
	NumberOfDirtiedPages,
	NumberOfWrittenPages,
	NumberOfAnonymousTransparentHugePages,
	NumberOfFreeCmaPages,
	
	/// Found in `/sys/devices/system/node/node<X>/vmstat` and `/sys/devices/system/node/node<X>/numastat` where `<X>` is a zero-based NUMA node number.
	NumaHit,
	NumaMiss,
	NumaForeign,
	NumaInterleaveHit,
	NumaLocalNode,
	NumaOtherNode,
	
	/// This is for all other possibilities.
	Unknown(String),
}

impl VirtualMemoryStatisticName
{
	#[inline]
	pub(crate) fn parse(name: &str) -> Self
	{
		use self::VirtualMemoryStatisticName::*;
		
		match name
		{
			"nr_free_pages" => NumberOFreePages,
			"nr_alloc_batch" => NumberOfBatchAllocatedPages,
			"nr_inactive_anon" => NumberOfInactiveAnonymousPages,
			"nr_active_anon" => NumberOfActiveAnonymousPages,
			"nr_inactive_file" => NumberOfInactiveFilePages,
			"nr_active_file" => NumberOfActiveFilePages,
			"nr_unevictable" => NumberOfUnevictablePages,
			"nr_mlock" => NumberOfLockedPages,
			"nr_anon_pages" => NumberOfAnonymousPages,
			"nr_mapped" => NumberOfMappedPages,
			"nr_file_pages" => NumberOfFilePages,
			"nr_dirty" => NumberOfDirtyPages,
			"nr_writeback" => NumberOfWritebackPages,
			"nr_slab_reclaimable" => NumberOfReclaimableSlabPages,
			"nr_slab_unreclaimable" => NumberOfUnreclaimableSlabPages,
			"nr_page_table_pages" => NumberOfPageTablePages,
			"nr_kernel_stack" => NumberOfKernelStackPages,
			"nr_unstable" => NumberOfUnstablePages,
			"nr_bounce" => NumberOfBouncePages,
			"nr_vmscan_write" => NumberOfVirtualMemoryWritePages,
			"nr_vmscan_immediate_reclaim" => NumberOfVirtualMemoryImmediateReclaimPages,
			"nr_writeback_temp" => NumberOfWritebackTemporaryPages,
			"nr_isolated_anon" => NumberOfIsolatedAnonymousPages,
			"nr_isolated_file" => NumberOfIsolatedFilePages,
			"nr_shmem" => NumberOfShmemPages,
			"nr_dirtied" => NumberOfDirtiedPages,
			"nr_written" => NumberOfWrittenPages,
			"nr_anon_transparent_hugepages" => NumberOfAnonymousTransparentHugePages,
			"nr_free_cma" => NumberOfFreeCmaPages,

			// found in '/sys/devices/system/node/nodeX/vmstat' and '/sys/devices/system/node/nodeX/numastat'
			"numa_hit" => NumaHit,
			"numa_miss" => NumaMiss,
			"numa_foreign" => NumaForeign,
			"interleave_hit" => NumaInterleaveHit,
			"local_node" => NumaLocalNode,
			"other_node" => NumaOtherNode,
			
			other @ _ => Unknown(other.to_owned()),
		}
	}
}
