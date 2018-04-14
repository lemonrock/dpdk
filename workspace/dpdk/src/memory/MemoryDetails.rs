// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryDetails
{
	physicalMemorySizeInBytes: u64,
	numberOfChannels: u31,
	numberOfRanks: u8,
}

impl MemoryDetails
{
	#[inline(always)]
	pub fn constantMemoryDetails() -> MemoryDetails
	{
		MemoryDetails
		{
			physicalMemorySizeInBytes: unsafe { rte_eal_get_physmem_size() },
			numberOfChannels: unsafe { rte_memory_get_nchannel() },
			numberOfRanks: (unsafe { rte_memory_get_nrank() }) as u8,
		}
	}

	#[inline(always)]
	pub fn dumpPhysicalMemoryLayoutToStandardError()
	{
		unsafe { rte_dump_physmem_layout(stderr as *mut FILE)}
	}
}
