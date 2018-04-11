// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_fdir_info
{
	pub mode: rte_fdir_mode,
	pub mask: rte_eth_fdir_masks,
	pub flex_conf: rte_eth_fdir_flex_conf,
	pub guarant_spc: u32,
	pub best_spc: u32,
	pub flow_types_mask: [u64; 1usize],
	pub max_flexpayload: u32,
	pub flex_payload_unit: u32,
	pub max_flex_payload_segment_num: u32,
	pub flex_payload_limit: u16,
	pub flex_bitmask_unit: u32,
	pub max_flex_bitmask_num: u32,
}

impl Default for rte_eth_fdir_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_fdir_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_fdir_info {{ mode: {:?}, mask: {:?}, flex_conf: {:?}, flow_types_mask: {:?} }}", self.mode, self.mask, self.flex_conf, self.flow_types_mask)
	}
}
