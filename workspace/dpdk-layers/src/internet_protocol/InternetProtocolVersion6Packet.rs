// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct InternetProtocolVersion6Packet
{
	/// Header.
	pub header: InternetProtocolVersion6PacketHeader,
}

impl Display for InternetProtocolVersion6Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl InternetProtocolVersion6Packet
{
	/// Use this to eliminate invalid traffic.
	#[inline(always)]
	pub(crate) fn is_packet_length_too_short(layer_3_length: u16) -> bool
	{
		layer_3_length < InternetProtocolVersion6PacketHeader::HeaderSizeU16
	}
	
	#[inline(always)]
	pub(crate) fn process<'a>(&'a mut self, packet: PacketBuffer, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		let header = &self.header;
		
		if unlikely!(header.is_version_not_6())
		{
			drop!(InternetProtocolVersion6HeaderIsNot6 { ethernet_addresses, header }, packet_processing, packet)
		}
		
		// TODO: Reject fragmentation of anything other than TCP / UDP traffic.
		xxx;
		
		// TODO: IPV6 header validation & extended header validation (which feeds into fragmented packet checks, which is currently very NAIVE). Look at rte_net.c's rte_net_skip_ip6_ext().
		xxx;
		let header_length_including_extension_headers = InternetProtocolVersion6PacketHeader::HeaderSizeU16 + xxxx;
		
		// TODO: Check layer 4 protocol number matches whether this can be unicast / broadcast / multicast.
		xxx;
		
		let packet = match packet_processing.reassemble_fragmented_internet_protocol_version_4_packet(packet, recent_timestamp, header, header_length_including_extension_headers)
		{
			None => return,
			Some(packet) => packet,
		};
		
		let source_address = &header.source_address;
		let destination_address = &header.destination_address;
		
		// TODO: Source address may be 0.0.0.0 for DHCPDISCOVER broadcast; there might be something analogous in IPv6?
		xxx;
		if unlikely(source_address.is_not_valid_unicast())
		{
			drop!(InternetProtocolVersion6SourceAddressNotValidUnicast { ethernet_addresses, header }, packet_processing, packet)
		}
		
		if unlikely(packet_processing.is_source_internet_protocol_version_6_address_denied(source_address))
		{
			drop!(InternetProtocolVersion6SourceAddressDenied { ethernet_addresses, header }, packet_processing, packet)
		}
		
		if destination_ethernet_address.is_valid_unicast()
		{
			if unlikely!(packet_processing.is_internet_protocol_version_6_host_address_not_one_of_ours(destination_address))
			{
				drop!(InternetProtocolVersion6UnicastDestinationIsNotUs { ethernet_addresses, header }, packet_processing, packet)
			}
			
			xxx;
		}
		else if let Some(lower_32_bits) = destination_ethernet_address.internet_protocol_version_6_multicast_32_bits()
		{
			if unlikely!(destination_address.does_not_have_multicast_prefix())
			{
				drop!(InternetProtocolVersion6MulticastAddressIsNotMulticast { ethernet_addresses, header }, packet_processing, packet)
			}
			
			if unlikely!(destination_address.does_not_have_lower_32_bits(lower_32_bits))
			{
				drop!(InternetProtocolVersion6MulticastAddressMismatchesEthernetAddress { ethernet_addresses, header }, packet_processing, packet)
			}
			
			if packet_processing.is_internet_protocol_version_6_multicast_address_not_one_of_ours(destination_address)
			{
				drop!(InternetProtocolVersion6MulticastAddressDenied { ethernet_addresses, header }, packet_processing, packet)
			}
			
			unsupported!("Multicast IPv6 packets are not supported");
			packet.free_direct_contiguous_packet();
			return
		}
		else
		{
			drop!(InternetProtocolVersion6DestinationWasLoopbackUnspecifiedOrDocumentationAddress { ethernet_addresses, header }, packet_processing, packet)
		}
	}
}
