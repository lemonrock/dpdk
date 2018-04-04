// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveSideScalingOffloadFlowType
{
	Raw = RTE_ETH_FLOW_RAW as u16,
	IpV4 = RTE_ETH_FLOW_IPV4 as u16,
	FragmentingIpV4 = RTE_ETH_FLOW_FRAG_IPV4 as u16,
	NonFragmentingIpV4Tcp = RTE_ETH_FLOW_NONFRAG_IPV4_TCP as u16,
	NonFragmentingIpV4Udp = RTE_ETH_FLOW_NONFRAG_IPV4_UDP as u16,
	NonFragmentingIpV4Sctp = RTE_ETH_FLOW_NONFRAG_IPV4_SCTP as u16,
	NonFragmentingIpV4Other = RTE_ETH_FLOW_NONFRAG_IPV4_OTHER as u16,
	IpV6 = RTE_ETH_FLOW_IPV6 as u16,
	FragmentingIpV6 = RTE_ETH_FLOW_FRAG_IPV6 as u16,
	NonFragmentingIpV6Tcp = RTE_ETH_FLOW_NONFRAG_IPV6_TCP as u16,
	NonFragmentingIpV6Udp = RTE_ETH_FLOW_NONFRAG_IPV6_UDP as u16,
	NonFragmentingIpV6Sctp = RTE_ETH_FLOW_NONFRAG_IPV6_SCTP as u16,
	NonFragmentingIpV6Other = RTE_ETH_FLOW_NONFRAG_IPV6_OTHER as u16,
	Layer2Payload = RTE_ETH_FLOW_L2_PAYLOAD as u16,
	
	// Not supported in test-pmd?
	IpV6Extended = RTE_ETH_FLOW_IPV6_EX as u16,
	IpV6TcpExtended = RTE_ETH_FLOW_IPV6_TCP_EX as u16,
	IpV6UdpExtended = RTE_ETH_FLOW_IPV6_UDP_EX as u16,
	Port = RTE_ETH_FLOW_PORT as u16,
	VXLAN = RTE_ETH_FLOW_VXLAN as u16,
	GENEVE = RTE_ETH_FLOW_GENEVE as u16,
	NVGRE = RTE_ETH_FLOW_NVGRE as u16,
}

impl ReceiveSideScalingOffloadFlowType
{
	// see str2flowtype() in cmdline.c
	pub fn fromTestPmdName(name: &str) -> Option<ReceiveSideScalingOffloadFlowType>
	{
		match name
		{
			"raw" => Some(ReceiveSideScalingOffloadFlowType::Raw),
			"ipv4" => Some(ReceiveSideScalingOffloadFlowType::IpV4),
			"ipv4-frag" => Some(ReceiveSideScalingOffloadFlowType::FragmentingIpV4),
			"ipv4-tcp" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV4Tcp),
			"ipv4-udp" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV4Udp),
			"ipv4-sctp" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV4Sctp),
			"ipv4-other" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV4Other),
			"ipv6" => Some(ReceiveSideScalingOffloadFlowType::IpV6),
			"ipv6-frag" => Some(ReceiveSideScalingOffloadFlowType::FragmentingIpV6),
			"ipv6-tcp" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV6Tcp),
			"ipv6-udp" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV6Udp),
			"ipv6-sctp" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV6Sctp),
			"ipv6-other" => Some(ReceiveSideScalingOffloadFlowType::NonFragmentingIpV6Other),
			"l2_payload" => Some(ReceiveSideScalingOffloadFlowType::Layer2Payload),
			
			_ => None,
		}
	}
}
