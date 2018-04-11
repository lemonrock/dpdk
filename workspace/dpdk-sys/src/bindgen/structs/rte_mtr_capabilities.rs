// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_mtr_capabilities
{
	pub n_max: u32,
	pub n_shared_max: u32,
	pub identical: c_int,
	pub shared_identical: c_int,
	pub shared_n_flows_per_mtr_max: u32,
	pub chaining_n_mtrs_per_flow_max: u32,
	pub chaining_use_prev_mtr_color_supported: c_int,
	pub chaining_use_prev_mtr_color_enforced: c_int,
	pub meter_srtcm_rfc2697_n_max: u32,
	pub meter_trtcm_rfc2698_n_max: u32,
	pub meter_trtcm_rfc4115_n_max: u32,
	pub meter_rate_max: u64,
	pub color_aware_srtcm_rfc2697_supported: c_int,
	pub color_aware_trtcm_rfc2698_supported: c_int,
	pub color_aware_trtcm_rfc4115_supported: c_int,
	pub policer_action_recolor_supported: c_int,
	pub policer_action_drop_supported: c_int,
	pub stats_mask: u64,
}

impl Default for rte_mtr_capabilities
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mtr_capabilities
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mtr_capabilities {{  }}")
	}
}
