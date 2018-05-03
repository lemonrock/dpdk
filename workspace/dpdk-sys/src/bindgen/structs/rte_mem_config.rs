// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct rte_mem_config
{
	pub magic: u32,
	pub nchannel: u32,
	pub nrank: u32,
	pub mlock: rte_rwlock_t,
	pub qlock: rte_rwlock_t,
	pub mplock: rte_rwlock_t,
	pub memory_hotplug_lock: rte_rwlock_t,
	pub memzones: rte_fbarray,
	pub memsegs: [rte_memseg_list; 64usize],
	pub tailq_head: [rte_tailq_head; 32usize],
	pub malloc_heaps: [malloc_heap; 8usize],
	pub mem_cfg_addr: u64,
}

impl Default for rte_mem_config
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

