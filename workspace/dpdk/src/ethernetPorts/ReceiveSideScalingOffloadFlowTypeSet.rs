// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// See also comments for ETH_RSS_IPV4 in rte_ethdev.h
bitflags!
{
	pub flags ReceiveSideScalingOffloadFlowTypeSet: u64
	{		
		const Raw = 1 << RTE_ETH_FLOW_RAW,
		const IpV4 = 1 << RTE_ETH_FLOW_IPV4,
		const FragmentingIpV4 = 1 << RTE_ETH_FLOW_FRAG_IPV4,
		const NonFragmentingIpV4Tcp = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_TCP,
		const NonFragmentingIpV4Udp = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_UDP,
		const NonFragmentingIpV4Sctp = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_SCTP,
		const NonFragmentingIpV4Other = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_OTHER,
		const IpV6 = 1 << RTE_ETH_FLOW_IPV6,
		const FragmentingIpV6 = 1 << RTE_ETH_FLOW_FRAG_IPV6,
		const NonFragmentingIpV6Tcp = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_TCP,
		const NonFragmentingIpV6Udp = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_UDP,
		const NonFragmentingIpV6Sctp = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_SCTP,
		const NonFragmentingIpV6Other = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_OTHER,
		const Layer2Payload = 1 << RTE_ETH_FLOW_L2_PAYLOAD,
		const IpV6Extended = 1 << RTE_ETH_FLOW_IPV6_EX,
		const IpV6TcpExtended = 1 << RTE_ETH_FLOW_IPV6_TCP_EX,
		const IpV6UdpExtended = 1 << RTE_ETH_FLOW_IPV6_UDP_EX,
		const Port = 1 << RTE_ETH_FLOW_PORT,
		const VXLAN = 1 << RTE_ETH_FLOW_VXLAN,
		const GENEVE = 1 << RTE_ETH_FLOW_GENEVE,
		const NVGRE = 1 << RTE_ETH_FLOW_NVGRE,
		
		const Ip = 1 << RTE_ETH_FLOW_IPV4
			| 1 << RTE_ETH_FLOW_FRAG_IPV4
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_OTHER
			| 1 << RTE_ETH_FLOW_IPV6
			| 1 << RTE_ETH_FLOW_FRAG_IPV6
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_OTHER
			| 1 << RTE_ETH_FLOW_IPV6_EX,
		
		const Udp = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_UDP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_UDP
			| 1 << RTE_ETH_FLOW_IPV6_UDP_EX,
		
		const Tcp = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_TCP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_TCP 
			| 1 << RTE_ETH_FLOW_IPV6_TCP_EX,
		 
		const Sctp = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_SCTP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_SCTP,
		
		const Tunnel = 1 << RTE_ETH_FLOW_VXLAN
			| 1 << RTE_ETH_FLOW_GENEVE
			| 1 << RTE_ETH_FLOW_NVGRE,
		
		const ProtocolMask = 1 << RTE_ETH_FLOW_IPV4
			| 1 << RTE_ETH_FLOW_FRAG_IPV4
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_TCP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_UDP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_SCTP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_OTHER
			| 1 << RTE_ETH_FLOW_IPV6
			| 1 << RTE_ETH_FLOW_FRAG_IPV6
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_TCP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_UDP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_SCTP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_OTHER
			| 1 << RTE_ETH_FLOW_L2_PAYLOAD
			| 1 << RTE_ETH_FLOW_IPV6_EX
			| 1 << RTE_ETH_FLOW_IPV6_TCP_EX
			| 1 << RTE_ETH_FLOW_IPV6_UDP_EX
			| 1 << RTE_ETH_FLOW_PORT 
			| 1 << RTE_ETH_FLOW_VXLAN
			| 1 << RTE_ETH_FLOW_GENEVE
			| 1 << RTE_ETH_FLOW_NVGRE,
		
		const All = 1 << RTE_ETH_FLOW_RAW
			| 1 << RTE_ETH_FLOW_IPV4
			| 1 << RTE_ETH_FLOW_FRAG_IPV4
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_TCP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_UDP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_SCTP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_OTHER
			| 1 << RTE_ETH_FLOW_IPV6
			| 1 << RTE_ETH_FLOW_FRAG_IPV6
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_TCP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_UDP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_SCTP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_OTHER
			| 1 << RTE_ETH_FLOW_L2_PAYLOAD
			| 1 << RTE_ETH_FLOW_IPV6_EX
			| 1 << RTE_ETH_FLOW_IPV6_TCP_EX
			| 1 << RTE_ETH_FLOW_IPV6_UDP_EX
			| 1 << RTE_ETH_FLOW_PORT 
			| 1 << RTE_ETH_FLOW_VXLAN
			| 1 << RTE_ETH_FLOW_GENEVE
			| 1 << RTE_ETH_FLOW_NVGRE,
	}
}

impl Default for ReceiveSideScalingOffloadFlowTypeSet
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

const NumberOfFieldsInDpdkApi: usize = 1;

impl ReceiveSideScalingOffloadFlowTypeSet
{
	#[inline(always)]
	pub fn asHashFilterSet(&self) -> [u32; NumberOfFieldsInDpdkApi]
	{
		[self.bits as u32]
	}
}
