// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_tm_node_capabilities_1_1
{
	pub sched_n_children_max: u32,
	pub sched_sp_n_priorities_max: u32,
	pub sched_wfq_n_children_per_group_max: u32,
	pub sched_wfq_n_groups_max: u32,
	pub sched_wfq_weight_max: u32,
}

impl Default for rte_tm_node_capabilities_1_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_tm_node_capabilities_1_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_tm_node_capabilities_1_1 {{  }}")
	}
}
