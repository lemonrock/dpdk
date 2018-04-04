// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Similar to rte_eth_tunnel_type enum, but without the baggage
// As of DPDK 16.07, on VxLan is actually usefully supported
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UdpTunnelType
{
	VxLan = 1,
	Geneve = 2,
	Teredo = 3,
}

impl UdpTunnelType
{
	#[inline(always)]
	pub fn defaultUdpPort(&self) -> u16
	{
		match *self
		{
			UdpTunnelType::VxLan => 4789,
			UdpTunnelType::Geneve => 6081,
			UdpTunnelType::Teredo => 3544,
		}
	}
	
	#[inline(always)]
	pub fn defaultUdpTunnelConfiguration(&self) -> UdpTunnelConfiguration
	{
		UdpTunnelConfiguration
		{
			udpPort: self.defaultUdpPort(),
			udpTunnelType: *self,
		}
	}
}
