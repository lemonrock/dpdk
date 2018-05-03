// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub union Layer3Packet
{
	/// An internet protocol (IP) version 4 packet.
	pub internet_protocol_version_4_packet: InternetProtocolVersion4Packet,
	
	/// An internet protocol (IP) version 6 packet.
	pub internet_protocol_version_6_packet: InternetProtocolVersion6Packet,
	
//	/// An address resolution protocol (ARP) packet.
//	pub address_resolution_protocol_packet: AddressResolutionProtocolPacket,
	
	/// Other kinds of layer 3 packets, not differentiated.
	pub other: PhantomData<u8>,
}

impl Layer3Packet
{
	#[inline(always)]
	pub(crate) fn process_internet_protocol_version_4(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, layer_3_length: u16, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
	{
		// TODO: Validate length
		
		if destination_ethernet_address.is_valid_unicast()
		{
			xxx;
		}
		else if destination_ethernet_address.is_broadcast()
		{
			// process a broadcast ipv4 packet (address must be 255.255.255.255)
			xxx;
		}
		else if let Some(lower_23_bits) = destination_ethernet_address.internet_protocol_version_4_multicast_23_bits()
		{
			if packet_processing_configuration.is_denied_internet_protocol_version_4_multicast_23_bits(lower_23_bits)
			{
				finish!(packet)
			}
			
			// process a multicast ipv4 packet - address must match, slightly ambiguously, the lower 23 bits.
			
			xxx;
		}
		else
		{
			finish!(packet)
		}
	}
	
	#[inline(always)]
	pub(crate) fn process_internet_protocol_version_6(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, layer_3_length: u16, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
	{
		// TODO: Validate length
		
		if destination_ethernet_address.is_valid_unicast()
		{
			xxx;
		}
		else if let Some(lower_32_bits) = destination_ethernet_address.internet_protocol_version_6_multicast_32_bits()
		{
			if packet_processing_configuration.is_denied_internet_protocol_version_6_multicast_32_bits(lower_32_bits)
			{
				finish!(packet)
			}
			
			// process a multicast ipv4 packet - validate address prefix.
			
			// Important if this a neighbour solicitation.
			
			xxx;
		}
		else
		{
			finish!(packet)
		}
	}
	
//	#[inline(always)]
//	pub(crate) fn process_address_resolution_protocol(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, layer_3_length: u16, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
//	{
//		if unlikely(layer_3_length < (size_of::<AddressResolutionProtocolPacketHeader>() as u16))
//		{
//			finish!(packet)
//		}
//
//		let address_resolution_protocol_packet = unsafe { &mut self.address_resolution_protocol_packet };
//
//		if unlikely(address_resolution_protocol_packet.is_header_invalid_for_internet_protocol_version_4(layer_3_length))
//		{
//			finish!(packet)
//		}
//
//		address_resolution_protocol_packet.process(packet, packet_processing_configuration, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
//	}
}
