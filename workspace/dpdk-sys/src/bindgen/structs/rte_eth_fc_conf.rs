// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_fc_conf
{
	pub high_water: u32,
	pub low_water: u32,
	pub pause_time: u16,
	pub send_xon: u16,
	pub mode: rte_eth_fc_mode,
	pub mac_ctrl_frame_fwd: u8,
	pub autoneg: u8,
}

impl Default for rte_eth_fc_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_fc_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_fc_conf {{ mode: {:?} }}", self.mode)
	}
}
