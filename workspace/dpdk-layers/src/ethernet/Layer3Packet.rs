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

impl Layer3Packet
{
	#[inline(always)]
	pub(crate) fn process_internet_protocol_version_4(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, layer_3_length: u16, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
	{
		if unlikely!(InternetProtocolVersion4Packet::is_packet_length_too_short(layer_3_length))
		{
			finish!(packet)
		}
		
		let header = &self.internet_protocol_version_4_packet.header;
		
		if unlikely!(header.is_version_not_4())
		{
			finish!(packet)
		}
		
		let total_length = header.total_length();
		
		if unlikely!(total_length != layer_3_length)
		{
			finish!(packet)
		}
		
		if unlikely!(header.has_invalid_fragmentation_flags_or_identification())
		{
			finish!(packet)
		}
		
		let header_length_including_options = header.header_length_including_options();
		
		if unlikely!(total_length < header_length_including_options as u16)
		{
			finish!(packet)
		}
		
		let header_has_ipv4_options = header_length_including_options != InternetProtocolVersion4PacketHeader::HeaderSizeU8;
		
		if likely!(header_has_ipv4_options)
		{
			if cfg!(feature = "drop-packets-with-ipv4-options")
			{
				finish!(packet)
			}
			else
			{
				let header_pointer = unsafe { header as *const InternetProtocolVersion4PacketHeader as usize };
				let mut options_pointer = header_pointer + InternetProtocolVersion4PacketHeader::HeaderSize;
				let end_of_options_pointer = header_pointer + (header_length_including_options as usize);
				while options_pointer != end_of_options_pointer
				{
					let increment = match unsafe { *(options_pointer as *const u8) }
					{
						// End-of-Options List; we do not validate that any subsequent padding is zeroed.
						0 =>
						{
							break
						},
						
						// No-Operation
						1 => 1,
						
						// Processing is NOT in compliance with RFC 7126; we simply ignore all options.
						// We do not validate that options are duplicated.
						unsupported @ _ =>
						{
							let length_pointer = options_pointer + 1;
							
							if unlikely!(length_pointer + 1 == end_of_options_pointer)
							{
								finish!(packet)
							}
							
							let length_including_option_kind_and_length_field = unsafe { *(length_pointer as *const u8) };
							
							if unlikely!(length_including_option_kind_and_length_field < 2)
							{
								finish!(packet)
							}
							
							let length_including_option_kind_and_length_field = length_including_option_kind_and_length_field as usize;
							
							if unlikely!(options_pointer + length_including_option_kind_and_length_field > end_of_options_pointer)
							{
								finish!(packet)
							}
							
							length_including_option_kind_and_length_field
						}
					};
					
					options_pointer += increment;
				}
			}
		}
		
		// NOTE: The header checksum is not validated. They are of limited benefit (indeed, they don't exist in version 6), and the assumption is made that they will nearly always be calculated by hardware offload, as nearly all modern network cards can do this. Data arriving via virtual drivers (eg TUN / TAP) will almost certainly have passed through an operating system's checksum validation.
		
		
		
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
		if unlikely!(InternetProtocolVersion6Packet::is_packet_length_too_short(layer_3_length))
		{
			finish!(packet)
		}
		
		let header = &self.internet_protocol_version_6_packet.header;
		
		if unlikely!(header.is_version_not_6())
		{
			finish!(packet)
		}
		
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
	
	#[inline(always)]
	pub(crate) fn process_address_resolution_protocol(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, layer_3_length: u16, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
	{
		if unlikely!(AddressResolutionProtocolPacket::is_packet_length_too_short(layer_3_length))
		{
			finish!(packet)
		}

		let address_resolution_protocol_packet = unsafe { &mut self.address_resolution_protocol_packet };

		if unlikely!(address_resolution_protocol_packet.is_invalid_for_internet_protocol_version_4(layer_3_length))
		{
			finish!(packet)
		}

		address_resolution_protocol_packet.process(packet, packet_processing_configuration, source_ethernet_address, destination_ethernet_address)
	}
}
