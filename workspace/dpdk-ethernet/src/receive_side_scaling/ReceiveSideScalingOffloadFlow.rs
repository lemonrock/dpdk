// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// See also comments for ETH_RSS_IPV4 in rte_ethdev.h
bitflags!
{
	pub struct ReceiveSideScalingOffloadFlow: u64
	{
		const Raw = 1 << RTE_ETH_FLOW_RAW;
		
		const InternetProtocolVersion4 = 1 << RTE_ETH_FLOW_IPV4;
		
		const FragmentingInternetProtocolVersion4 = 1 << RTE_ETH_FLOW_FRAG_IPV4;
		
		const NonFragmentingInternetProtocolVersion4TransmissionControlProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_TCP;
		
		const NonFragmentingInternetProtocolVersion4UserDatagramProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_UDP;
		
		const NonFragmentingInternetProtocolVersion4StreamControlTransmissionProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_SCTP;
		
		const NonFragmentingInternetProtocolVersion4Other = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_OTHER;
		
		const InternetProtocolVersion6 = 1 << RTE_ETH_FLOW_IPV6;
		
		const FragmentingInternetProtocolVersion6 = 1 << RTE_ETH_FLOW_FRAG_IPV6;
		
		const NonFragmentingInternetProtocolVersion6TransmissionControlProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_TCP;
		
		const NonFragmentingInternetProtocolVersion6UserDatagramProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_UDP;
		
		const NonFragmentingInternetProtocolVersion6StreamControlTransmissionProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_SCTP;
		
		const NonFragmentingInternetProtocolVersion6Other = 1 << RTE_ETH_FLOW_NONFRAG_IPV6_OTHER;
		
		const Layer2Payload = 1 << RTE_ETH_FLOW_L2_PAYLOAD;
		
		const InternetProtocolVersion6Extended = 1 << RTE_ETH_FLOW_IPV6_EX;
		
		const InternetProtocolVersion6TransmissionControlProtocolExtended = 1 << RTE_ETH_FLOW_IPV6_TCP_EX;
		
		const InternetProtocolVersion6UserDatagramProtocolExtended = 1 << RTE_ETH_FLOW_IPV6_UDP_EX;
		
		const Port = 1 << RTE_ETH_FLOW_PORT;
		
		/// Virtual eXtensible Local Area Network (VXLAN) tunnel.
		const VXLAN = 1 << RTE_ETH_FLOW_VXLAN;
		
		/// Generic Network Virtualization Encapsulation (GENEVE) tunnel
		const GENEVE = 1 << RTE_ETH_FLOW_GENEVE;
		
		/// Network Virtualization using Generic Routing Encapsulation (NVGRE) tunnel.
		const NVGRE = 1 << RTE_ETH_FLOW_NVGRE;
		
		const InternetProtocol = 1 << RTE_ETH_FLOW_IPV4
			| 1 << RTE_ETH_FLOW_FRAG_IPV4
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV4_OTHER
			| 1 << RTE_ETH_FLOW_IPV6
			| 1 << RTE_ETH_FLOW_FRAG_IPV6
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_OTHER
			| 1 << RTE_ETH_FLOW_IPV6_EX;
		
		const UserDatagramProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_UDP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_UDP
			| 1 << RTE_ETH_FLOW_IPV6_UDP_EX;
		
		const TransmissionControlProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_TCP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_TCP
			| 1 << RTE_ETH_FLOW_IPV6_TCP_EX;
		 
		const StreamControlTransmissionProtocol = 1 << RTE_ETH_FLOW_NONFRAG_IPV4_SCTP
			| 1 << RTE_ETH_FLOW_NONFRAG_IPV6_SCTP;
		
		const Tunnel = 1 << RTE_ETH_FLOW_VXLAN
			| 1 << RTE_ETH_FLOW_GENEVE
			| 1 << RTE_ETH_FLOW_NVGRE;
		
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
			| 1 << RTE_ETH_FLOW_NVGRE;
		
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
			| 1 << RTE_ETH_FLOW_NVGRE;
	}
}
