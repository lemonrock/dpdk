// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet processing configuration for a particular combination of Outer Virtual LAN tag, Inner Virtual LAN tag and (our valid unicast) Ethernet Address.
#[derive(Debug)]
pub struct PacketProcessing<PPDO: PacketProcessingDropObserver>
{
	/// Inner 802.1Q Virtual LAN honour drop eligible.
	inner_honour_drop_eligible_indicator: bool,
	
	/// Inner 802.1Q Virtual LAN permitted classes of service.
	inner_permitted_classes_of_service: PermittedClassesOfService,
	
	/// Our unicast ethernet addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_unicast_ethernet_address: MediaAccessControlAddress,
	
	/// Blacklist or whitelist of ethernet addresses.
	source_ethernet_address_blacklist_or_whitelist: MediaAccessControlAddressList,
	
	/// Our unicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_4_host_addresses: HashSet<InternetProtocolVersion4HostAddress>,
	
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_6_host_addresses: HashSet<InternetProtocolVersion6HostAddress>,
	
	/// Our multicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_4_multicast_addresses: HashSet<InternetProtocolVersion4HostAddress>,
	
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_6_multicast_addresses: HashSet<InternetProtocolVersion6HostAddress>,
	
	denied_source_internet_protocol_version_4_host_addresses: InternetProtocolVersion4LongestPrefixMatchTable,
	
	denied_source_internet_protocol_version_6_host_addresses: InternetProtocolVersion6LongestPrefixMatchTable,

	dropped_packet_reporting: Rc<PPDO>,
}

impl<DPO: PacketProcessingDropObserver> PacketProcessing<DPO>
{
	#[inline(always)]
	pub(crate) fn dropped_packet(&self, reason: PacketProcessingDropReason)
	{
		self.dropped_packet_reporting.dropped_packet(reason)
	}
	
	#[inline(always)]
	pub(crate) fn honour_drop_eligible_indicator(&self, drop_eligible_indicator: bool) -> bool
	{
		drop_eligible_indicator && self.inner_honour_drop_eligible_indicator
	}
	
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
	pub(crate) fn is_internet_protocol_version_4_multicast_address_not_one_of_ours(&self, internet_protocol_version_4_multicast_address: InternetProtocolVersion4HostAddress) -> bool
	{
		const MulticastIsUnsupportedAtThisTime: bool = false;
		
		MulticastIsUnsupportedAtThisTime
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_6_host_address_not_one_of_our_multicast_addresses(&self, internet_protocol_version_6_multicast_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		// TODO: solicited node check implicit group membership.
		// TODO: all nodes (FF02::1); equivalent to 224.0.0.1 and 255.255.255.255.
		
		const MulticastIsUnsupportedAtThisTime: bool = false;
		
		MulticastIsUnsupportedAtThisTime
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
	pub(crate) fn is_internet_protocol_version_6_host_address_one_of_ours(&self, internet_protocol_version_6_host_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_6_host_address.is_valid_unicast(), "internet_protocol_version_6_host_address '{:?}' is not valid unicast", internet_protocol_version_6_host_address);
		
		self.our_valid_internet_protocol_version_6_host_addresses.contains(&internet_protocol_version_6_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_6_host_address_not_one_of_our_unicast_addresses(&self, internet_protocol_version_6_host_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_6_host_address.is_valid_unicast(), "internet_protocol_version_6_host_address '{:?}' is not valid unicast", internet_protocol_version_6_host_address);
		
		!self.is_internet_protocol_version_6_host_address_one_of_ours(internet_protocol_version_6_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_source_internet_protocol_version_4_address_denied(&self, internet_protocol_version_4_host_address: &InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(is_source_internet_protocol_version_4_address_denied.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		self.denied_source_internet_protocol_version_4_host_addresses.look_up(internet_protocol_version_4_host_address).is_some()
	}
	
	#[inline(always)]
	pub(crate) fn is_source_internet_protocol_version_6_address_denied(&self, internet_protocol_version_6_host_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		debug_assert!(is_source_internet_protocol_version_4_address_denied.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		self.denied_source_internet_protocol_version_4_host_addresses.look_up(internet_protocol_version_4_host_address).is_some()
	}
	
	#[inline(always)]
	pub(crate) fn reassemble_fragmented_internet_protocol_version_4_packet(&self, packet: impl Packet, recent_timestamp: Cycles, internet_protocol_version_4_packet_header: &mut InternetProtocolVersion4PacketHeader, header_length_including_options: u16) -> Option<PacketBuffer>
	{
		let table = unsafe { &mut * self.internet_protocol_version_4_packet_reassembly_table.get() };
		let result = table.reassemble_fragmented_internet_protocol_version_4_packet(packet, recent_timestamp, internet_protocol_version_4_packet_header);
		table.if_death_row_is_full_free_all_packets_on_death_row();
		result
	}
	
	#[inline(always)]
	pub(crate) fn reassemble_fragmented_internet_protocol_version_6_packet(&self, packet: impl Packet, recent_timestamp: Cycles, internet_protocol_version_6_packet_header: &mut InternetProtocolVersion6PacketHeader, header_length_including_extension_headers: u16) -> Option<PacketBuffer>
	{
		let table = unsafe { &mut * self.internet_protocol_version_6_packet_reassembly_table.get() };
		let result = table.reassemble_fragmented_internet_protocol_version_6_packet(packet, recent_timestamp, internet_protocol_version_6_packet_header);
		table.if_death_row_is_full_free_all_packets_on_death_row();
		result
	}
	
	#[inline(always)]
	pub(crate) fn add_to_address_resolution_cache(&self, sender_hardware_address: &MediaAccessControlAddress, sender_protocol_address: InternetProtocolVersion4HostAddress, packet: impl Packet)
	{
		// TODO: Manage an ARP cache.
		unsupported!("ARP: adding to resolution cache");
		packet.free_direct_contiguous_packet();
	}
}
