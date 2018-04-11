// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_mtr_params
{
	pub meter_profile_id: u32,
	pub use_prev_mtr_color: c_int,
	pub dscp_table: *mut rte_mtr_color,
	pub meter_enable: c_int,
	pub action: [rte_mtr_policer_action; 3usize],
	pub stats_mask: u64,
}

impl Default for rte_mtr_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mtr_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mtr_params {{ dscp_table: {:?}, action: {:?} }}", self.dscp_table, self.action)
	}
}
