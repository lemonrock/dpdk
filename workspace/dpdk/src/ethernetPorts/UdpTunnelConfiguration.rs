// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UdpTunnelConfiguration
{
	pub udpPort: u16,
	pub udpTunnelType: UdpTunnelType,
}

impl UdpTunnelConfiguration
{
	#[inline(always)]
	pub fn as_rte_eth_udp_tunnel(&self) -> rte_eth_udp_tunnel
	{
		rte_eth_udp_tunnel
		{
			udp_port: self.udpPort,
			prot_type: self.udpTunnelType as u8,
		}
	}
}
