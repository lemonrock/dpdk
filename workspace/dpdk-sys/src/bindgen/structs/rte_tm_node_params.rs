// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_tm_node_params
{
	pub shaper_profile_id: u32,
	pub shared_shaper_id: *mut u32,
	pub n_shared_shapers: u32,
	pub __bindgen_anon_1: rte_tm_node_params__bindgen_ty_1,
	pub stats_mask: u64,
}

impl Default for rte_tm_node_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_tm_node_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_tm_node_params {{ shared_shaper_id: {:?}, __bindgen_anon_1: {:?} }}", self.shared_shaper_id, self.__bindgen_anon_1)
	}
}
