// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_fdir_flow_ext
{
	pub vlan_tci: u16,
	pub flexbytes: [u8; 16usize],
	pub is_vf: u8,
	pub dst_id: u16,
}

impl Default for rte_eth_fdir_flow_ext
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_fdir_flow_ext
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_fdir_flow_ext {{ flexbytes: {:?} }}", self.flexbytes)
	}
}
