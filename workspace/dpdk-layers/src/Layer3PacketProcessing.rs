// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension trait for a Layer 3 packet.
pub trait Layer3PacketProcessing
{
	/// Process an Internet Protocol (IP) version 4 packet.
	#[inline(always)]
	fn process_internet_protocol_version_4<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses);
	
	/// Process an Internet Protocol (IP) version 6 packet.
	#[inline(always)]
	fn process_internet_protocol_version_6<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses);
	
	/// Process an Address Resolution Protocol (ARP) packet.
	#[inline(always)]
	fn process_address_resolution_protocol<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses);
}

impl Layer3PacketProcessing for Layer3Packet
{
	#[inline(always)]
	fn process_internet_protocol_version_4<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		if unlikely!(InternetProtocolVersion4Packet::is_packet_length_too_short(layer_3_length))
		{
			drop!(InternetProtocolVersion4PacketIsTooShort { ethernet_addresses }, packet_processing, packet)
		}
		
		let internet_protocol_version_4_packet: &'a mut InternetProtocolVersion4Packet = unsafe { transmute(&mut self.other) };
		
		internet_protocol_version_4_packet.process(packet, packet_processing, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	fn process_internet_protocol_version_6<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		if unlikely!(InternetProtocolVersion6Packet::is_packet_length_too_short(layer_3_length))
		{
			drop!(InternetProtocolVersion6PacketIsTooShort { ethernet_addresses }, packet_processing, packet)
		}
		
		let internet_protocol_version_6_packet: &'a mut InternetProtocolVersion6Packet = unsafe { transmute(&mut self.other) };
		
		internet_protocol_version_6_packet.process(packet, packet_processing, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	fn process_address_resolution_protocol<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		if unlikely!(AddressResolutionProtocolPacket::is_packet_length_too_short(layer_3_length))
		{
			drop!(AddressResolutionProtocolPacketIsTooShort { ethernet_addresses }, packet_processing, packet)
		}
		
		let address_resolution_protocol_packet: &'a mut AddressResolutionProtocolPacket = unsafe { transmute(&mut self.other) };
		
		if unlikely!(address_resolution_protocol_packet.is_invalid_for_internet_protocol_version_4(layer_3_length))
		{
			drop!(AddressResolutionProtocolNotSupportedForAnythingOtherThanInternetProtocolVersion4 { ethernet_addresses }, packet_processing, packet)
		}
		
		address_resolution_protocol_packet.process(packet, packet_processing, ethernet_addresses)
	}
}
