// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_tunnel_filter_conf
{
	pub outer_mac: ether_addr,
	pub inner_mac: ether_addr,
	pub inner_vlan: u16,
	pub ip_type: rte_tunnel_iptype,
	pub ip_addr: rte_eth_tunnel_filter_conf_1,
	pub filter_type: u16,
	pub tunnel_type: rte_eth_tunnel_type,
	pub tenant_id: u32,
	pub queue_id: u16,
}

impl Default for rte_eth_tunnel_filter_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_tunnel_filter_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_tunnel_filter_conf {{ outer_mac: {:?}, inner_mac: {:?}, ip_type: {:?}, ip_addr: {:?}, tunnel_type: {:?} }}", self.outer_mac, self.inner_mac, self.ip_type, self.ip_addr, self.tunnel_type)
	}
}
