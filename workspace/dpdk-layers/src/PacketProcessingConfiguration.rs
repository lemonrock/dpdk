// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet processing configuration for a particular combination of Outer Virtual LAN tag, Inner Virtual LAN tag and (our valid unicast) Ethernet Address.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PacketProcessingConfiguration
{
	/// Inner 802.1Q Virtual LAN permitted classes of service.
	pub inner_permitted_classes_of_service: PermittedClassesOfService,
	
	/// Our unicast ethernet addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	pub our_valid_unicast_ethernet_address: MediaAccessControlAddress,
	
	/// Blacklist or whitelist of ethernet addresses.
	pub source_ethernet_address_blacklist_or_whitelist: MediaAccessControlAddressList,
	
	/// Our unicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	pub our_valid_internet_protocol_version_4_host_addresses: HashSet<InternetProtocolVersion4HostAddress>,
}

impl PacketProcessingConfiguration
{
	#[inline(always)]
	pub(crate) fn drop_packets_of_class_of_service(&self, class_of_service: ClassOfService) -> bool
	{
		self.inner_permitted_classes_of_service.is_denied(class_of_service)
	}
	
	#[inline(always)]
	pub(crate) fn is_ethernet_address_our_valid_unicast_ethernet_address(&self, destination_ethernet_address: &MediaAccessControlAddress) -> bool
	{
		debug_assert!(destination_ethernet_address.is_valid_unicast(), "ethernet_address '{:?}' is not valid unicast", destination_ethernet_address);
		
		&self.our_valid_unicast_ethernet_address == destination_ethernet_address
	}
	
	#[inline(always)]
	pub(crate) fn is_ethernet_address_not_our_valid_unicast_ethernet_address(&self, destination_ethernet_address: &MediaAccessControlAddress) -> bool
	{
		debug_assert!(destination_ethernet_address.is_valid_unicast(), "destination_ethernet_address '{:?}' is not valid unicast", destination_ethernet_address);
		
		&self.our_valid_unicast_ethernet_address != destination_ethernet_address
	}
	
	#[inline(always)]
	pub(crate) fn is_denied_source_ethernet_address(&self, source_ethernet_address: &MediaAccessControlAddress) -> bool
	{
		debug_assert!(source_ethernet_address.is_valid_unicast(), "source_ethernet_address '{:?}' is not valid unicast", source_ethernet_address);
		
		self.source_ethernet_address_blacklist_or_whitelist.is_denied(&source_ethernet_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_denied_internet_protocol_version_4_multicast_23_bits(&self, lower_23_bits: &[u8; 3]) -> bool
	{
		const NoMulticastAddressesAreSupportedAtThisTime: bool = false;
		
		NoMulticastAddressesAreSupportedAtThisTime
	}
	
	#[inline(always)]
	pub(crate) fn is_denied_internet_protocol_version_6_multicast_32_bits(&self, lower_32_bits: &[u8; 4]) -> bool
	{
		const NoMulticastAddressesAreSupportedAtThisTime: bool = false;
		
		NoMulticastAddressesAreSupportedAtThisTime
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_4_host_address_one_of_ours(&self, internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_4_host_address.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		self.our_valid_internet_protocol_version_4_host_addresses.contains(&internet_protocol_version_4_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_4_host_address_not_one_of_ours(&self, internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_4_host_address.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		!self.is_internet_protocol_version_4_host_address_one_of_ours(internet_protocol_version_4_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn internet_protocol_version_4_host_address_conflict(&self, packet: PacketBuffer)
	{
		// TODO: Handle ARP host address conflicts.
		eprintln!("ARP is not supported");
		finish!(packet)
	}
	
	#[inline(always)]
	pub(crate) fn add_to_address_resolution_cache(&self, sender_hardware_address: &MediaAccessControlAddress, sender_protocol_address: InternetProtocolVersion4HostAddress)
	{
		// TODO: Manage an ARP cache.
		eprintln!("ARP is not supported");
	}
}
