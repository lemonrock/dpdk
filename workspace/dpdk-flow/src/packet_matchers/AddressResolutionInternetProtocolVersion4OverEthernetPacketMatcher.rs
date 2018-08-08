// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A matcher that matches an Address Resolution Protocol (ARP) Internet Protocol (IP) version 4 packet over Etherenet.
///
/// The underlying DPDK functionality supports other kinds of ARP packets but these are very rare in practice.
#[derive(Debug)]
#[repr(transparent)]
pub struct AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	underlying: rte_flow_item_arp_eth_ipv4,
}

impl Clone for AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		generic_clone(&self)
	}
}

impl PartialEq for AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		generic_equals(&self, &rhs)
	}
}

impl Eq for AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
}

impl PartialOrd for AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		generic_compare(&self, &rhs)
	}
}

impl Hash for AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		generic_hash::<H, _>(self, hasher)
	}
}

impl PacketMatcher for AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	type DpdkType = rte_flow_item_arp_eth_ipv4;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_ARP_ETH_IPV4;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_arp_eth_ipv4_mask }
	}
}

impl AddressResolutionInternetProtocolVersion4OverEthernetPacketMatcher
{
	/// A `source_ethernet_address` of 0xFFFFFF matches all Ethernet source addresses.
	/// A `destination_ethernet_address` of 0xFFFFFF matches all Ethernet destination addresses.
	/// A `source_internet_protocol_version_4_address` of 0xFFFFFFFF matches all Internet Protocol (IP) version 4 source addresses.
	/// A `destination_internet_protocol_version_4_address` of 0xFFFFFFFF matches all Internet Protocol (IP) version 4 destination addresses.
	#[inline(always)]
	pub fn new(source_ethernet_address: MediaAccessControlAddress, destination_ethernet_address: MediaAccessControlAddress, source_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, destination_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, operation: Operation) -> Self
	{
		Self
		{
			underlying: rte_flow_item_arp_eth_ipv4
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
