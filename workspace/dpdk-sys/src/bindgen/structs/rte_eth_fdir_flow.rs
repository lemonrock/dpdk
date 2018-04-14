// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_fdir_flow
{
	pub l2_flow: BindgenUnionField<rte_eth_l2_flow>,
	pub udp4_flow: BindgenUnionField<rte_eth_udpv4_flow>,
	pub tcp4_flow: BindgenUnionField<rte_eth_tcpv4_flow>,
	pub sctp4_flow: BindgenUnionField<rte_eth_sctpv4_flow>,
	pub ip4_flow: BindgenUnionField<rte_eth_ipv4_flow>,
	pub udp6_flow: BindgenUnionField<rte_eth_udpv6_flow>,
	pub tcp6_flow: BindgenUnionField<rte_eth_tcpv6_flow>,
	pub sctp6_flow: BindgenUnionField<rte_eth_sctpv6_flow>,
	pub ipv6_flow: BindgenUnionField<rte_eth_ipv6_flow>,
	pub mac_vlan_flow: BindgenUnionField<rte_eth_mac_vlan_flow>,
	pub tunnel_flow: BindgenUnionField<rte_eth_tunnel_flow>,
	pub bindgen_union_field: [u32; 11usize],
}

impl Default for rte_eth_fdir_flow
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_fdir_flow
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_fdir_flow {{ union }}")
	}
}
