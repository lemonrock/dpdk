// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumaNodeStatisticName
{
	// found in '/sys/devices/system/node/nodeX/vmstat'
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
	
	// found in '/sys/devices/system/node/nodeX/vmstat' and '/sys/devices/system/node/nodeX/numastat'
	NumaHit,
	NumaMiss,
	NumaForeign,
	NumaInterleaveHit,
	NumaLocalNode,
	NumaOtherNode,
	
	Unknown(String),
}

impl NumaNodeStatisticName
{
	#[inline]
	pub fn parse(name: &str) -> NumaNodeStatisticName
	{
		match name
		{
			"nr_free_pages" => NumaNodeStatisticName::NumberOFreePages,
			"nr_alloc_batch" => NumaNodeStatisticName::NumberOfBatchAllocatedPages,
			"nr_inactive_anon" => NumaNodeStatisticName::NumberOfInactiveAnonymousPages,
			"nr_active_anon" => NumaNodeStatisticName::NumberOfActiveAnonymousPages,
			"nr_inactive_file" => NumaNodeStatisticName::NumberOfInactiveFilePages,
			"nr_active_file" => NumaNodeStatisticName::NumberOfActiveFilePages,
			"nr_unevictable" => NumaNodeStatisticName::NumberOfUnevictablePages,
			"nr_mlock" => NumaNodeStatisticName::NumberOfLockedPages,
			"nr_anon_pages" => NumaNodeStatisticName::NumberOfAnonymousPages,
			"nr_mapped" => NumaNodeStatisticName::NumberOfMappedPages,
			"nr_file_pages" => NumaNodeStatisticName::NumberOfFilePages,
			"nr_dirty" => NumaNodeStatisticName::NumberOfDirtyPages,
			"nr_writeback" => NumaNodeStatisticName::NumberOfWritebackPages,
			"nr_slab_reclaimable" => NumaNodeStatisticName::NumberOfReclaimableSlabPages,
			"nr_slab_unreclaimable" => NumaNodeStatisticName::NumberOfUnreclaimableSlabPages,
			"nr_page_table_pages" => NumaNodeStatisticName::NumberOfPageTablePages,
			"nr_kernel_stack" => NumaNodeStatisticName::NumberOfKernelStackPages,
			"nr_unstable" => NumaNodeStatisticName::NumberOfUnstablePages,
			"nr_bounce" => NumaNodeStatisticName::NumberOfBouncePages,
			"nr_vmscan_write" => NumaNodeStatisticName::NumberOfVirtualMemoryWritePages,
			"nr_vmscan_immediate_reclaim" => NumaNodeStatisticName::NumberOfVirtualMemoryImmediateReclaimPages,
			"nr_writeback_temp" => NumaNodeStatisticName::NumberOfWritebackTemporaryPages,
			"nr_isolated_anon" => NumaNodeStatisticName::NumberOfIsolatedAnonymousPages,
			"nr_isolated_file" => NumaNodeStatisticName::NumberOfIsolatedFilePages,
			"nr_shmem" => NumaNodeStatisticName::NumberOfShmemPages,
			"nr_dirtied" => NumaNodeStatisticName::NumberOfDirtiedPages,
			"nr_written" => NumaNodeStatisticName::NumberOfWrittenPages,
			"nr_anon_transparent_hugepages" => NumaNodeStatisticName::NumberOfAnonymousTransparentHugePages,
			"nr_free_cma" => NumaNodeStatisticName::NumberOfFreeCmaPages,

			// found in '/sys/devices/system/node/nodeX/vmstat' and '/sys/devices/system/node/nodeX/numastat'
			"numa_hit" => NumaNodeStatisticName::NumaHit,
			"numa_miss" => NumaNodeStatisticName::NumaMiss,
			"numa_foreign" => NumaNodeStatisticName::NumaForeign,
			"interleave_hit" => NumaNodeStatisticName::NumaInterleaveHit,
			"local_node" => NumaNodeStatisticName::NumaLocalNode,
			"other_node" => NumaNodeStatisticName::NumaOtherNode,
			
			other @ _ => NumaNodeStatisticName::Unknown(other.to_owned()),
		}
	}
}
