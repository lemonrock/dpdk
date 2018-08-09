// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification for an `PacketMatcher::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct EthernetHeaderMask
{
	/// Source and destination addresses.
	pub ethernet_addresses: EthernetAddressesMask,
	
	/// Operation mask.
	pub ether_type_or_legacy_ethernet_frame_size: NetworkEndianU16,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_eth,
}

custom_deserialize!
{
	EthernetHeaderMask,
	0 => ethernet_addresses,
	1 => ether_type_or_legacy_ethernet_frame_size,
}

impl MaskedPacketMatcher for EthernetHeaderMask
{
	type Type = rte_flow_item_eth;
}

impl Mask for EthernetHeaderMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		&self.cached
	}
}

impl EthernetHeaderMask
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(ethernet_addresses: EthernetAddressesMask, ether_type_or_legacy_ethernet_frame_size: NetworkEndianU16) -> Self
	{
		Self
		{
			cached: rte_flow_item_eth
			{
				dst: ethernet_addresses.destination_ethernet_address.to_ether_addr(),
				src: ethernet_addresses.source_ethernet_address.to_ether_addr(),
				type_: ether_type_or_legacy_ethernet_frame_size.to_network_endian(),
			},
			ethernet_addresses,
			ether_type_or_legacy_ethernet_frame_size,
		}
	}
}
