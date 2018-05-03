// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct rte_config
{
	pub master_lcore: u32,
	pub lcore_count: u32,
	pub numa_node_count: u32,
	pub numa_nodes: [u32; 8usize],
	pub service_lcore_count: u32,
	pub lcore_role: [rte_lcore_role_t; 128usize],
	pub process_type: rte_proc_type_t,
	pub iova_mode: rte_iova_mode,
	pub mem_config: *mut rte_mem_config,
}

impl Default for rte_config
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

