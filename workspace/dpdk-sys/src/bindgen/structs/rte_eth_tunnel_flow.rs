// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_tunnel_flow
{
	pub tunnel_type: rte_eth_fdir_tunnel_type,
	pub tunnel_id: u32,
	pub mac_addr: ether_addr,
}

impl Default for rte_eth_tunnel_flow
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_tunnel_flow
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_tunnel_flow {{ tunnel_type: {:?}, mac_addr: {:?} }}", self.tunnel_type, self.mac_addr)
	}
}
