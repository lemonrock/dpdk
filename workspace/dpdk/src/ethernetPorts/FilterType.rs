// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Based on the enum rte_filter_type but without the noise and invalid values
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilterType
{
	MediaAccessControlVirtualLan,
	EtherType,
	Flexible,
	TcpSyn,
	NTuple,
	VirtualLanTunnel,
	FlowDirector,
	Hash,
	Layer2Tunnel,
}

impl FilterType
{
	pub const All: [FilterType; 9] =
	[
		FilterType::MediaAccessControlVirtualLan,
		FilterType::EtherType,
		FilterType::Flexible,
		FilterType::TcpSyn,
		FilterType::NTuple,
		FilterType::VirtualLanTunnel,
		FilterType::FlowDirector,
		FilterType::Hash,
		FilterType::Layer2Tunnel,
	];
	
	#[inline(always)]
	pub fn as_rte_filter_type(&self) -> rte_filter_type
	{
		match *self
		{
			FilterType::MediaAccessControlVirtualLan => rte_filter_type::RTE_ETH_FILTER_MACVLAN,
			FilterType::EtherType => rte_filter_type::RTE_ETH_FILTER_ETHERTYPE,
			FilterType::Flexible => rte_filter_type::RTE_ETH_FILTER_FLEXIBLE,
			FilterType::TcpSyn => rte_filter_type::RTE_ETH_FILTER_SYN,
			FilterType::NTuple => rte_filter_type::RTE_ETH_FILTER_NTUPLE,
			FilterType::VirtualLanTunnel => rte_filter_type::RTE_ETH_FILTER_TUNNEL,
			FilterType::FlowDirector => rte_filter_type::RTE_ETH_FILTER_FDIR,
			FilterType::Hash => rte_filter_type::RTE_ETH_FILTER_HASH,
			FilterType::Layer2Tunnel => rte_filter_type::RTE_ETH_FILTER_L2_TUNNEL,
		}
	}
}
