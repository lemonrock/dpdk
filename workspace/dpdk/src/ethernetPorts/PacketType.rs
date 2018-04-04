// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

// Refer to rte_mbuf.h, lines 228 - 721
bitflags!
{
	pub flags PacketType: u32
	{
		const Layer2Ether = RTE_PTYPE_L2_ETHER,
		const Layer2EtherTimeSync = RTE_PTYPE_L2_ETHER_TIMESYNC,
		const Layer2EtherArp = RTE_PTYPE_L2_ETHER_ARP,
		const Layer2EtherLldp = RTE_PTYPE_L2_ETHER_LLDP,
		const Layer2EtherNsh = RTE_PTYPE_L2_ETHER_NSH,
		const Layer2EtherQinQ = RTE_PTYPE_L2_ETHER_QINQ,
		const Layer2EtherVirtualLan = RTE_PTYPE_L2_ETHER_VLAN,
		
		const Layer3IpV4 = RTE_PTYPE_L3_IPV4,
		const Layer3IpV4Extended = RTE_PTYPE_L3_IPV4_EXT,
		const Layer3IpV6 = RTE_PTYPE_L3_IPV6,
		const Layer3IpV4ExtendedUnknown = RTE_PTYPE_L3_IPV4_EXT_UNKNOWN,
		const Layer3IpV6Extended = RTE_PTYPE_L3_IPV6_EXT,
		const Layer3IpV6ExtendedUnknown = RTE_PTYPE_L3_IPV6_EXT_UNKNOWN,
		
		const Layer4Tcp = RTE_PTYPE_L4_TCP,
		const Layer4Udp = RTE_PTYPE_L4_UDP,
		const Layer4Fragmented = RTE_PTYPE_L4_FRAG,
		const Layer4Sctp = RTE_PTYPE_L4_SCTP,
		const Layer4Icmp = RTE_PTYPE_L4_ICMP,
		const Layer4NonFragmented = RTE_PTYPE_L4_NONFRAG,
		
		const TunnelIp = RTE_PTYPE_TUNNEL_IP,
		const TunnelGre = RTE_PTYPE_TUNNEL_GRE,
		const TunnelVxLan = RTE_PTYPE_TUNNEL_VXLAN,
		const TunnelNvGre = RTE_PTYPE_TUNNEL_NVGRE,
		const TunnelGeneve = RTE_PTYPE_TUNNEL_GENEVE,
		const TunnelGrenNat = RTE_PTYPE_TUNNEL_GRENAT,
		
		const InnerLayer2Ether = RTE_PTYPE_INNER_L2_ETHER,
		const InnerLayer2EtherQinQ = RTE_PTYPE_INNER_L2_ETHER_QINQ,
		const InnerLayer2EtherVirtualLan = RTE_PTYPE_INNER_L2_ETHER_VLAN,
		const InnerLayer3IpV4 = RTE_PTYPE_INNER_L3_IPV4,
		const InnerLayer3IpV4Extended = RTE_PTYPE_INNER_L3_IPV4_EXT,
		const InnerLayer3IpV6 = RTE_PTYPE_INNER_L3_IPV6,
		const InnerLayer3IpV4ExtendedUnknown = RTE_PTYPE_INNER_L3_IPV4_EXT_UNKNOWN,
		const InnerLayer3IpV6Extended = RTE_PTYPE_INNER_L3_IPV6_EXT,
		const InnerLayer3IpV6ExtendedUnknown = RTE_PTYPE_INNER_L3_IPV6_EXT_UNKNOWN,
		const InnerLayer4Tcp = RTE_PTYPE_INNER_L4_TCP,
		const InnerLayer4Udp = RTE_PTYPE_INNER_L4_UDP,
		const InnerLayer4Fragmented = RTE_PTYPE_INNER_L4_FRAG,
		const InnerLayer4Sctp = RTE_PTYPE_INNER_L4_SCTP,
		const InnerLayer4Icmp = RTE_PTYPE_INNER_L4_ICMP,
		const InnerLayer4NonFragmented = RTE_PTYPE_INNER_L4_NONFRAG,
	}
}

impl Default for PacketType
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl PacketType
{
	// Equivalent to DPDK's RTE_ETH_IS_IPV4_HDR(ptype)
	#[inline(always)]
	pub fn isIpV4Header(self) -> bool
	{
		self.bits & RTE_PTYPE_L3_IPV4 != 0
	}
	
	// Equivalent to DPDK's RTE_ETH_IS_IPV6_HDR(ptype)
	#[inline(always)]
	pub fn isIpV6Header(self) -> bool
	{
		self.bits & RTE_PTYPE_L3_IPV6 != 0
	}
	
	#[inline(always)]
	pub fn isNotValidEther(self) -> bool
	{
		self.bits & RTE_PTYPE_L2_ETHER != RTE_PTYPE_L2_ETHER
	}
	
	#[inline(always)]
	pub fn isEtherVirtualLan(self) -> bool
	{
		self.bits & RTE_PTYPE_L2_ETHER_VLAN == RTE_PTYPE_L2_ETHER_VLAN
	}

	#[inline(always)]
	pub fn isEtherQinQ(self) -> bool
	{
		self.bits & RTE_PTYPE_L2_ETHER_QINQ == RTE_PTYPE_L2_ETHER_QINQ
	}
}
