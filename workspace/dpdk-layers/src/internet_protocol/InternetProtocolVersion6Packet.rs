// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct InternetProtocolVersion6Packet
{
	/// Header.
	pub header: InternetProtocolVersion6PacketHeader,
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
	pub(crate) fn process<'a>(&'a mut self, packet: PacketBuffer, packet_processing: &PacketProcessing<&impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses<'a>)
	{
		let header = &self.header;
		
		if unlikely!(header.is_version_not_6())
		{
			drop!(InternetProtocolVersion6HeaderIsNot6 { ethernet_addresses, header }, packet_processing, packet)
		}
		
		xxx;
		// TODO: IPV6 header validation
		
		// TODO: fragmentation
		
		// TODO: Equivalent of packet_processing.our_valid_internet_protocol_version_4_host_addresses and is_internet_protocol_version_4_host_address_one_of_ours/is_internet_protocol_version_4_host_address_not_one_of_ours
		// TODO Actually use those mthods!
		
		if destination_ethernet_address.is_valid_unicast()
		{
			xxx;
		}
		else if let Some(lower_32_bits) = destination_ethernet_address.internet_protocol_version_6_multicast_32_bits()
		{
			if packet_processing.is_denied_internet_protocol_version_6_multicast_32_bits(lower_32_bits)
			{
				drop!(InternetProtocolVersion6MulticastAddressDenied { ethernet_addresses, header }, packet_processing, packet)
			}
			
			// process a multicast ipv4 packet - validate address prefix.
			
			// Important if this a neighbour solicitation.
			
			xxx;
		}
		else
		{
			drop!(InternetProtocolVersion6MulticastAddressWrong { ethernet_addresses, header }, packet_processing, packet)
		}
	}
}
