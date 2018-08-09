// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification for an `PacketMatcher::InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisement`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementSpecification
{
	/// Header check sum.
	pub header_check_sum: InternetCheckSum,
	
	/// Target address.
	pub target_address: InternetProtocolVersion6HostAddress,
	
	/// Flags.
	pub flags: InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementFlags,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_icmp6_nd_na,
}

custom_deserialize!
{
	InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementSpecification,
	0 => header_check_sum,
	1 => target_address,
	2 => flags,
}

impl MaskedPacketMatcher for InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementSpecification
{
	type Type = rte_flow_item_icmp6_nd_na;
}

impl Specification for InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementSpecification
{
	const DpdkFlowType: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_ICMP6_ND_NA;
	
	type Mask = InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementMask;
	
	#[inline(always)]
	fn dpdk_specification(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		&self.cached
	}
}

impl InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementSpecification
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(header_check_sum: InternetCheckSum, target_address: InternetProtocolVersion6HostAddress, flags: InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementFlags) -> Self
	{
		Self
		{
			header_check_sum,
			target_address,
			flags,
			cached: rte_flow_item_icmp6_nd_na
			{
				type_: InternetControlMessageProtocolVersion6Type::NeighborAdvertisement.into(),
				code: 0,
				checksum: header_check_sum.to_network_endian(),
				rso_reserved: flags.bits() as u32,
				target_addr: target_address.to_bytes(),
			}
		}
	}
}
