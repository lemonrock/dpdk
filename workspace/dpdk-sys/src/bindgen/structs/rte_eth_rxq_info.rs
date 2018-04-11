// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_eth_rxq_info
{
	pub mp: *mut rte_mempool,
	pub conf: rte_eth_rxconf,
	pub scattered_rx: u8,
	pub nb_desc: u16,
	pub __bindgen_padding_0: [u32; 9usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_eth_rxq_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_rxq_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_rxq_info {{ mp: {:?}, conf: {:?} }}", self.mp, self.conf)
	}
}
