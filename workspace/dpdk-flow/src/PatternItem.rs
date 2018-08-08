// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct MaskedPacketMatcherFields<T>
{
	from_specification: T,
	to_specification: Option<T>,
	mask: T,
}

/// Do not construct these directly, but use the associated methods starting `new_`, eg `new_address_resolution_protocol_for_internet_protocol_version_4_over_ethernet()`.
pub enum PacketMatcher
{
	/// A matcher that matches an Address Resolution Protocol (ARP) Internet Protocol (IP) version 4 packet over Ethernet.
	///
	/// The underlying DPDK functionality supports other kinds of ARP headers but always assumes an InternetProtocolVersion4-sized payload!
	AddressResolutionProtocolForInternetProtocolVersion4OverEthernet(MaskedPacketMatcherFields<rte_flow_item_arp_eth_ipv4>),
	
	#[allow(doc_missing)]
	Any(MaskedPacketMatcherFields<rte_flow_item_any>),
}

/// Specification for an `PacketMatcher::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
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
	
	#[serde(skip)]
	cached: UnsafeCell<Option<rte_flow_item_arp_eth_ipv4>>,
}

impl AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	pub fn new(source_ethernet_address: MediaAccessControlAddress, destination_ethernet_address: MediaAccessControlAddress, source_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, destination_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, operation: Operation) -> Self
	{
		let mut this = Self
		{
			source_ethernet_address,
			destination_ethernet_address,
			source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address,
			operation,
			cached: None,
		};
	}
	
	#[inline(always)]
	pub(crate) fn to_rte_flow_item_arp_eth_ipv4(self) -> &rte_flow_item_arp_eth_ipv4
	{
		self.populate_cached().as_ref().unwrap()
	}
	
	#[inline(always)]
	fn populate_cached(&self) -> &mut Option<rte_flow_item_arp_eth_ipv4>
	{
		let cached = self.cached();
		if cached.is_some()
		{
			return
		}
		*cached = Some
		(
			rte_flow_item_arp_eth_ipv4
			{
				hrd: HardwareType::Ethernet2.to_network_endian(),
				pro: EtherType::AddressResolutionProtocol.to_network_endian(),
				hln: MediaAccessControlAddress::SizeU8,
				pln: InternetProtocolVersion4HostAddress::SizeU8,
				op: self.operation.to_network_endian(),
				sha: self.source_ethernet_address.to_ether_addr(),
				spa: self.source_internet_protocol_version_4_address.as_network_endian(),
				tha: self.destination_ethernet_address.to_ether_addr(),
				tpa: self.destination_internet_protocol_version_4_address.as_network_endian(),
			}
		);
		cached
	}
	
	#[inline(always)]
	fn cached(&self) -> &mut Option<rte_flow_item_arp_eth_ipv4>
	{
		unsafe { &mut * self.cached.get() }
	}
}

/// Mask for an `PacketMatcher::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
///
/// Values are big-endian (network endian).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask
{
	/// Source ethernet address.
	pub source_ethernet_address: [u8; 6],
	
	/// Destination ethernet address.
	pub destination_ethernet_address: [u8; 6],
	
	/// Source internet protocol version 4 address.
	pub source_internet_protocol_version_4_address: NetworkEndianU32,
	
	/// Destination internet protocol version 4 address.
	pub destination_internet_protocol_version_4_address: NetworkEndianU32,
	
	/// Operation; recommended to be either Request or Reply.
	pub operation: NetworkEndianU16,
}

impl AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	pub(crate) fn to_rte_flow_item_arp_eth_ipv4(self) -> rte_flow_item_arp_eth_ipv4
	{
		rte_flow_item_arp_eth_ipv4
		{
			hrd: HardwareType::Ethernet2.to_network_endian(),
			pro: EtherType::AddressResolutionProtocol.to_network_endian(),
			hln: MediaAccessControlAddress::SizeU8,
			pln: InternetProtocolVersion4HostAddress::SizeU8,
			op: self.operation.to_network_endian(),
			sha: self.source_ethernet_address.to_ether_addr(),
			spa: self.source_internet_protocol_version_4_address.as_network_endian(),
			tha: self.destination_ethernet_address.to_ether_addr(),
			tpa: self.destination_internet_protocol_version_4_address.as_network_endian(),
		}
	}
}

impl PacketMatcher
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new_address_resolution_protocol_for_internet_protocol_version_4_over_ethernet(from_specification: AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification, to_specification: Option<AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification>, mask: ) -> Self
	{
		PacketMatcher::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet
		(
			MaskedPacketMatcherFields
			{
				from_specification: ,
				to_specification: to_specification.map(|specification|),
				mask: x,
			}
		)
		{
			underlying:
		}
	}
}


/*

An array of rte_flow_item terminated with the 'END' type.

It's not explicitly stated, but presumably the lifetime of the pointers is not an issue.

struct rte_flow_item {
	enum rte_flow_item_type type; /**< Item type. */
	const void *spec; /**< Pointer to item specification structure. */
	const void *last; /**< Defines an inclusive range (spec to last). */
	const void *mask; /**< Bit-mask applied to spec and last. */
};

struct rte_flow *
rte_flow_create(uint16_t port_id,
		const struct rte_flow_attr *attr,
		const struct rte_flow_item pattern[],
		const struct rte_flow_action actions[],
		struct rte_flow_error *error);

*/


impl AnyMaskedPacketMatcher
{
	/// Creates a new instance.
	///
	/// If `number_of_layers_covered` is zero then matches any layer.
	#[inline(always)]
	pub fn new(number_of_layers_covered: u32) -> Self
	{
		Self
		{
			underlying: rte_flow_item_any
			{
				num: number_of_layers_covered,
			}
		}
	}
}
