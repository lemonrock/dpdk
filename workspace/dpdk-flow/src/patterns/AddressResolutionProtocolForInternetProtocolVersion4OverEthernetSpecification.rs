// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification for an `Pattern::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	/// Source ethernet address.
	pub source_ethernet_address: MediaAccessControlAddress,
	
	/// Destination ethernet address.
	pub destination_ethernet_address: MediaAccessControlAddress,
	
	/// Source internet protocol version 4 address.
	pub source_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress,
	
	/// Destination internet protocol version 4 address.
	pub destination_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress,
	
	/// Operation; recommended to be either Request or Reply.
	pub operation: Operation,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_arp_eth_ipv4,
}

custom_deserialize!
{
	AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification,
	0 => source_ethernet_address,
	1 => destination_ethernet_address,
	2 => source_internet_protocol_version_4_address,
	3 => destination_internet_protocol_version_4_address,
	4 => operation,
}

impl Clone for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			source_ethernet_address: self.source_ethernet_address,
			destination_ethernet_address: self.destination_ethernet_address,
			source_internet_protocol_version_4_address: self.source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address: self.destination_internet_protocol_version_4_address,
			operation: self.operation,
			cached: bitwise_clone!(self, rte_flow_item_arp_eth_ipv4),
		}
	}
}

impl PartialOrd for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.source_ethernet_address.cmp(&rhs.source_ethernet_address).then_with(|| self.destination_ethernet_address.cmp(&rhs.destination_ethernet_address)).then_with(|| self.source_internet_protocol_version_4_address.cmp(&rhs.source_internet_protocol_version_4_address)).then_with(|| self.destination_internet_protocol_version_4_address.cmp(&rhs.destination_internet_protocol_version_4_address)).then_with(|| self.operation.cmp(&rhs.operation))
	}
}

impl PartialEq for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.source_ethernet_address == rhs.source_ethernet_address && self.destination_ethernet_address == rhs.destination_ethernet_address && self.source_internet_protocol_version_4_address == rhs.source_internet_protocol_version_4_address && self.destination_internet_protocol_version_4_address == rhs.destination_internet_protocol_version_4_address && self.operation == rhs.operation
	}
}

impl Eq for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
}

impl Hash for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.source_ethernet_address.hash(hasher);
		self.destination_ethernet_address.hash(hasher);
		self.source_internet_protocol_version_4_address.hash(hasher);
		self.destination_internet_protocol_version_4_address.hash(hasher);
		self.operation.hash(hasher)
	}
}

impl MaskedPattern for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	type Type = rte_flow_item_arp_eth_ipv4;
}

impl Specification for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	const DpdkFlowType: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_ARP_ETH_IPV4;
	
	type Mask = AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask;
	
	#[inline(always)]
	fn dpdk_specification(&self) -> &<Self as MaskedPattern>::Type
	{
		&self.cached
	}
}

impl AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(source_ethernet_address: MediaAccessControlAddress, destination_ethernet_address: MediaAccessControlAddress, source_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, destination_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, operation: Operation) -> Self
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
				spa: source_internet_protocol_version_4_address.as_network_endian(),
				tha: destination_ethernet_address.to_ether_addr(),
				tpa: destination_internet_protocol_version_4_address.as_network_endian(),
			}
		}
	}
}
