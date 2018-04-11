// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_fdir_masks
{
	pub vlan_tci_mask: u16,
	pub ipv4_mask: rte_eth_ipv4_flow,
	pub ipv6_mask: rte_eth_ipv6_flow,
	pub src_port_mask: u16,
	pub dst_port_mask: u16,
	pub mac_addr_byte_mask: u8,
	pub tunnel_id_mask: u32,
	pub tunnel_type_mask: u8,
}

impl Default for rte_eth_fdir_masks
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_fdir_masks
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_fdir_masks {{ ipv4_mask: {:?}, ipv6_mask: {:?} }}", self.ipv4_mask, self.ipv6_mask)
	}
}
