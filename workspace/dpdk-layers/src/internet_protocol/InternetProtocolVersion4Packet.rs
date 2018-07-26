// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct InternetProtocolVersion4Packet
{
	/// Header.
	pub header: InternetProtocolVersion4PacketHeader,
	
	/// Options.
	pub options: PhantomData<u8>,
	
	/// Payload.
	pub payload: Layer4Packet,
}

impl Display for InternetProtocolVersion4Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl InternetProtocolVersion4Packet
{
	#[inline(always)]
	pub(crate) fn is_packet_length_too_short(layer_3_length: u16) -> bool
	{
		layer_3_length < InternetProtocolVersion4PacketHeader::HeaderSizeU16
	}
	
	#[inline(always)]
	pub(crate) fn process<'a>(&'a mut self, packet: PacketBuffer, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		let header = &self.header;
		
		if unlikely!(header.is_version_not_4())
		{
			drop!(InternetProtocolVersion4HeaderIsNot4 { ethernet_addresses, header }, packet_processing, packet)
		}
		
		let total_length = header.total_length();
		
		if unlikely!(total_length != layer_3_length)
		{
			drop!(InternetProtocolVersion4TotalLengthInvalid { ethernet_addresses, header }, packet_processing, packet)
		}
		
		if unlikely!(header.has_invalid_fragmentation_flags_or_identification())
		{
			drop!(InternetProtocolVersion4InvalidFragmentationFlagsOrIdentification { ethernet_addresses, header }, packet_processing, packet)
		}
		
		let header_length_including_options = header.header_length_including_options();
		
		let header_length_including_options_as_u16 = header_length_including_options as u16;
		
		if unlikely!(total_length < header_length_including_options as u16)
		{
			drop!(InternetProtocolVersion4TotalLengthLessThanHeader { ethernet_addresses, header }, packet_processing, packet)
		}
		
		let header_has_ipv4_options = header_length_including_options != InternetProtocolVersion4PacketHeader::HeaderSizeU8;
		
		if likely!(header_has_ipv4_options)
		{
			if cfg!(feature = "drop-packets-with-ipv4-options")
			{
				drop!(InternetProtocolVersion4HasOptions { ethernet_addresses, header }, packet_processing, packet)
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
								drop!(InternetProtocolVersion4OptionLacksKind { ethernet_addresses, header }, packet_processing, packet)
							}
							
							let length_including_option_kind_and_length_field = unsafe { *(length_pointer as *const u8) };
							
							if unlikely!(length_including_option_kind_and_length_field < 2)
							{
								drop!(InternetProtocolVersion4OptionLengthTooShort { ethernet_addresses, header }, packet_processing, packet)
							}
							
							let length_including_option_kind_and_length_field = length_including_option_kind_and_length_field as usize;
							
							if unlikely!(options_pointer + length_including_option_kind_and_length_field > end_of_options_pointer)
							{
								drop!(InternetProtocolVersion4OptionLengthTooLong { ethernet_addresses, header }, packet_processing, packet)
							}
							
							length_including_option_kind_and_length_field
						}
					};
					
					options_pointer += increment;
				}
			}
		}
		
		// TODO: Overly small fragments, eg fragments smaller than MSS / MTU minima (eg 1280 for IPv6).
		
		// TODO: Reject fragmentation of anything other than TCP / UDP traffic.
		xxx;
		
		// TODO: The header checksum is not validated.
		xxx;
		
		// TODO: Check layer 4 protocol number matches whether this can be unicast / broadcast / multicast.
		xxx;
		
		let packet = match packet_processing.reassemble_fragmented_internet_protocol_version_4_packet(packet, recent_timestamp, header, header_length_including_options_as_u16)
		{
			None => return,
			Some(packet) => packet,
		};
		
		let source_address = header.source_address;
		let destination_address = header.destination_address;
		
		// TODO: Source address may be 0.0.0.0 for DHCPDISCOVER broadcast.
		xxx;
		if unlikely(source_address.is_not_valid_unicast())
		{
			drop!(InternetProtocolVersion4SourceAddressNotValidUnicast { ethernet_addresses, header }, packet_processing, packet)
		}
		
		if unlikely(packet_processing.is_source_internet_protocol_version_4_address_denied(&source_address))
		{
			drop!(InternetProtocolVersion4SourceAddressDenied { ethernet_addresses, header }, packet_processing, packet)
		}
		
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		
		if destination_ethernet_address.is_valid_unicast()
		{
			if unlikely!(packet_processing.is_internet_protocol_version_4_host_address_not_one_of_ours(destination_address))
			{
				drop!(InternetProtocolVersion4UnicastDestinationIsNotUs { ethernet_addresses, header }, packet_processing, packet)
			}
			
			xxx;
		}
		else if destination_ethernet_address.is_broadcast()
		{
			if unlikely!(header.destination_address.is_not_broadcast())
			{
				drop!(InternetProtocolVersion4EthernetBroadcastNotInternetBroadcast { ethernet_addresses, header }, packet_processing, packet)
			}
			
			unsupported!("Broadcast IPv4 packets are not supported");
			packet.free_direct_contiguous_packet();
			return
		}
		else if let Some(lower_23_bits) = destination_ethernet_address.internet_protocol_version_4_multicast_23_bits()
		{
			if unlikely!(destination_address.is_not_multicast())
			{
				drop!(InternetProtocolVersion4MulticastAddressIsNotMulticast { ethernet_addresses, header }, packet_processing, packet)
			}
			
			if unlikely!(destination_address.does_not_have_lower_23_bits(lower_23_bits))
			{
				drop!(InternetProtocolVersion4MulticastAddressMismatchesEthernetAddress { ethernet_addresses, header }, packet_processing, packet)
			}
			
			if packet_processing.is_internet_protocol_version_4_multicast_address_not_one_of_ours(destination_address)
			{
				drop!(InternetProtocolVersion4MulticastAddressDenied { ethernet_addresses, header }, packet_processing, packet)
			}
			
			unsupported!("Multicast IPv4 packets are not supported");
			packet.free_direct_contiguous_packet();
			return
		}
		else if source_address.is_unspecified()
		{
			xxx;
		}
		else
		{
			drop!(InternetProtocolVersion4DestinationWasLoopbackOrDocumentationAddress { ethernet_addresses, header }, packet_processing, packet)
		}
	}
}
