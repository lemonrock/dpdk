// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_tm_level_capabilities
{
	pub n_nodes_max: u32,
	pub n_nodes_nonleaf_max: u32,
	pub n_nodes_leaf_max: u32,
	pub non_leaf_nodes_identical: c_int,
	pub leaf_nodes_identical: c_int,
	pub _1: rte_tm_level_capabilities_1,
}

impl Default for rte_tm_level_capabilities
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_tm_level_capabilities
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_tm_level_capabilities {{ _1: {:?} }}", self._1)
	}
}
