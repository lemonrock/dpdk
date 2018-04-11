// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_fdir_flex_conf
{
	pub nb_payloads: u16,
	pub nb_flexmasks: u16,
	pub flex_set: [rte_eth_flex_payload_cfg; 8usize],
	pub flex_mask: [rte_eth_fdir_flex_mask; 22usize],
}

impl Default for rte_eth_fdir_flex_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_fdir_flex_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_fdir_flex_conf {{ flex_set: {:?}, flex_mask: {:?} }}", self.flex_set, self.flex_mask)
	}
}
