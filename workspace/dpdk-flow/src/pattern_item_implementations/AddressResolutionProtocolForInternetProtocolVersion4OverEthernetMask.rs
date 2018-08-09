// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask for an `Pattern::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask
{
	/// Source ethernet address mask.
	pub source_ethernet_address: MediaAccessControlAddressMask,
	
	/// Destination ethernet address mask.
	pub destination_ethernet_address: MediaAccessControlAddressMask,
	
	/// Source internet protocol version 4 address mask.
	pub source_internet_protocol_version_4_address: NetworkEndianU32,
	
	/// Destination internet protocol version 4 address mask.
	pub destination_internet_protocol_version_4_address: NetworkEndianU32,
	
	/// Operation mask.
	pub operation: NetworkEndianU16,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_arp_eth_ipv4,
}

custom_deserialize!
{
	AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask,
	0 => source_ethernet_address,
	1 => destination_ethernet_address,
	2 => source_internet_protocol_version_4_address,
	3 => destination_internet_protocol_version_4_address,
	4 => operation,
}

bitwise_clone_partial_ord_ord_partial_eq_eq_hash!(AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask);

impl MaskedPattern for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask
{
	type Type = rte_flow_item_arp_eth_ipv4;
}

impl Mask for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPattern>::Type
	{
		&self.cached
	}
}

impl AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(source_ethernet_address: MediaAccessControlAddressMask, destination_ethernet_address: MediaAccessControlAddressMask, source_internet_protocol_version_4_address: NetworkEndianU32, destination_internet_protocol_version_4_address: NetworkEndianU32, operation: NetworkEndianU16) -> Self
	{
		Self
		{
			source_ethernet_address,
			destination_ethernet_address,
			source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address,
			operation,
			cached: rte_flow_item_arp_eth_ipv4
			{
				hrd: HardwareType::Ethernet2.to_network_endian(),
				pro: EtherType::AddressResolutionProtocol.to_network_endian(),
				hln: MediaAccessControlAddress::SizeU8,
				pln: InternetProtocolVersion4HostAddress::SizeU8,
				op: operation.to_network_endian(),
				sha: source_ethernet_address.to_ether_addr(),
				spa: source_internet_protocol_version_4_address.to_network_endian(),
				tha: destination_ethernet_address.to_ether_addr(),
				tpa: destination_internet_protocol_version_4_address.to_network_endian(),
			}
		}
	}
}
