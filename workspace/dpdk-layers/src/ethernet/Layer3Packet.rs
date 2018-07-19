// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
///
/// Note that Internet protocol version 4 packet header checksums are not validated unless done by hardware offload.
#[repr(C, packed)]
pub union Layer3Packet
{
	/// An internet protocol (IP) version 4 packet.
	pub internet_protocol_version_4_packet: InternetProtocolVersion4Packet,
	
	/// An internet protocol (IP) version 6 packet.
	pub internet_protocol_version_6_packet: InternetProtocolVersion6Packet,
	
	/// An address resolution protocol (ARP) packet.
	pub address_resolution_protocol_packet: AddressResolutionProtocolPacket,
	
	/// Other kinds of layer 3 packets, not differentiated.
	pub other: PhantomData<u8>,
}

impl Display for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(layer 3 packet)")
	}
}

impl Layer3Packet
{
	#[inline(always)]
	pub(crate) fn process_internet_protocol_version_4<'a>(&'a mut self, packet: PacketBuffer, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		if unlikely!(InternetProtocolVersion4Packet::is_packet_length_too_short(layer_3_length))
		{
			drop!(InternetProtocolVersion4PacketIsTooShort { ethernet_addresses }, packet_processing, packet)
		}
		
		let internet_protocol_version_4_packet = unsafe { &mut self.internet_protocol_version_4_packet };
		
		internet_protocol_version_4_packet.process(packet, packet_processing, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	pub(crate) fn process_internet_protocol_version_6<'a>(&'a mut self, packet: PacketBuffer, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		if unlikely!(InternetProtocolVersion6Packet::is_packet_length_too_short(layer_3_length))
		{
			drop!(InternetProtocolVersion6PacketIsTooShort { ethernet_addresses }, packet_processing, packet)
		}
		
		let internet_protocol_version_6_packet = unsafe { &mut self.internet_protocol_version_6_packet };
		
		internet_protocol_version_6_packet.process(packet, packet_processing, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	pub(crate) fn process_address_resolution_protocol<'a>(&'a mut self, packet: PacketBuffer, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		if unlikely!(AddressResolutionProtocolPacket::is_packet_length_too_short(layer_3_length))
		{
			drop!(AddressResolutionProtocolPacketIsTooShort { ethernet_addresses }, packet_processing, packet)
		}

		let address_resolution_protocol_packet = unsafe { &mut self.address_resolution_protocol_packet };

		if unlikely!(address_resolution_protocol_packet.is_invalid_for_internet_protocol_version_4(layer_3_length))
		{
			drop!(AddressResolutionProtocolNotSupportedForAnythingOtherThanInternetProtocolVersion4 { ethernet_addresses }, packet_processing, packet)
		}

		address_resolution_protocol_packet.process(packet, packet_processing, ethernet_addresses)
	}
}
