// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet processing.
pub trait InternetProtocolVersion6PacketHeaderProcessing
{
	/// Process.
	#[inline(always)]
	fn process<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses);
	
	#[doc(hidden)]
	#[inline(always)]
	fn process_extension_headers<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses, address_support: Layer4ProtocolNeedsToSupport);
}

impl InternetProtocolVersion6PacketHeaderProcessing for InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn process<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		let header = self;
		
		if unlikely!(self.is_version_not_6())
		{
			drop!(InternetProtocolVersion6HeaderIsNot6 { ethernet_addresses, header }, packet_processing, packet)
		}
		
		if cfg!(feature = "drop-ipv6-packets-with-non-zero-flow-label")
		{
			if unlikely!(self.flow_label() != 0)
			{
				drop!(InternetProtocolVersion6FlowLabelIsNonZero { ethernet_addresses, header }, packet_processing, packet)
			}
		}
		
		if unlikely!(packet_processing.is_source_internet_protocol_version_6_address_denied(&self.source_address))
		{
			drop!(InternetProtocolVersion6SourceAddressDenied { ethernet_addresses, header }, packet_processing, packet)
		}
		
		let destination_internet_protocol_version_6_address = &self.destination_address;
		
		if unlikely!(destination_internet_protocol_version_6_address.is_unspecified())
		{
			drop!(InternetProtocolVersion6DestinationAddressUnspecified { ethernet_addresses, header }, packet_processing, packet)
		}
		
		if unlikely!(destination_internet_protocol_version_6_address.is_documentation())
		{
			drop!(InternetProtocolVersion6DestinationAddressDocumentation { ethernet_addresses, header }, packet_processing, packet)
		}
		
		if unlikely!(destination_internet_protocol_version_6_address.is_loopback())
		{
			drop!(InternetProtocolVersion6DestinationAddressLoopback { ethernet_addresses, header }, packet_processing, packet)
		}
		
		let destination_ethernet_address = &ethernet_addresses.destination;
		
		let address_support = if destination_ethernet_address.is_valid_unicast()
		{
			if unlikely!(packet_processing.is_internet_protocol_version_6_host_address_not_one_of_our_unicast_addresses(destination_internet_protocol_version_6_address))
			{
				drop!(InternetProtocolVersion6UnicastDestinationIsNotUs { ethernet_addresses, header }, packet_processing, packet)
			}
			
			Layer4ProtocolNeedsToSupport::Unicast
		}
		else if let Some(lower_32_bits) = destination_ethernet_address.internet_protocol_version_6_multicast_32_bits()
		{
			match destination_internet_protocol_version_6_address.is_multicast()
			{
				None => drop!(InternetProtocolVersion6MulticastAddressIsNotMulticast { ethernet_addresses, header }, packet_processing, packet),
				
				Some(Err(parsing_error)) => drop!(InternetProtocolVersion6MulticastAddressIsNotValidMulticast { ethernet_addresses, header, parsing_error }, packet_processing, packet),
				
				Some(Ok((_lifetime, scope))) =>
				{
					if unlikely!(scope.is_interface_local_also_known_as_loopback())
					{
						drop!(InternetProtocolVersion6DestinationAddressInterfaceLocal { ethernet_addresses, header }, packet_processing, packet)
					}
					
					if unlikely!(destination_internet_protocol_version_6_address.does_not_have_lower_32_bits(lower_32_bits))
					{
						drop!(InternetProtocolVersion6MulticastAddressMismatchesEthernetAddress { ethernet_addresses, header }, packet_processing, packet)
					}
					
					if packet_processing.is_internet_protocol_version_6_host_address_not_one_of_our_multicast_addresses(destination_internet_protocol_version_6_address)
					{
						drop!(InternetProtocolVersion6MulticastAddressDenied { ethernet_addresses, header }, packet_processing, packet)
					}
					
					Layer4ProtocolNeedsToSupport::Multicast
				}
			}
		}
		else if unlikely!(destination_ethernet_address.is_broadcast())
		{
			drop!(InternetProtocolVersion6EthernetBroadcastShouldNotOccur { ethernet_addresses, header }, packet_processing, packet);
		}
		else
		{
			drop!(InternetProtocolVersion6DestinationWasLoopbackOrDocumentationAddress { ethernet_addresses, header }, packet_processing, packet)
		};
		
		self.process_extension_headers(packet, packet_processing, layer_3_length, ethernet_addresses, address_support)
	}
	
	#[inline(always)]
	fn process_extension_headers<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses, address_support: Layer4ProtocolNeedsToSupport)
	{
		macro_rules! process_type_length_value_options
		{
			($self: ident, $packet: ident, $packet_processing: ident, $extension_header_pointer: ident, $end_layer3_pointer: ident, $ethernet_addresses: ident) =>
			{
				{
					const MinimumTypeLengthValueExtensionHeaderSize: usize = 8;
					if unlikely!($extension_header_pointer + MinimumTypeLengthValueExtensionHeaderSize > $end_layer3_pointer)
					{
						drop!(InternetProtocolVersion6HopByHopOptionsUnderflow { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
					}
					
					let header_extension_length = extension_header_length_in_bytes($extension_header_pointer);
					if unlikely!($extension_header_pointer + header_extension_length >= $end_layer3_pointer)
					{
						drop!(InternetProtocolVersion6HopByHopOptionsHeaderExtensionLengthOverflow { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
					}
					
					let start_of_options_pointer = $extension_header_pointer + 2;
					let end_of_options_pointer = $extension_header_pointer + header_extension_length;
					let option_pointer = start_of_options_pointer;
					while option_pointer != end_of_options_pointer
					{
						if unlikely!(option_pointer + 1 > end_of_options_pointer)
						{
							drop!(InternetProtocolVersion6TypeLengthValueOptionTypeUnderflow { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
						}
						let option_type = option_type(option_pointer);
						
						const Pad1: u8 = 0x00;
						if option_type == Pad1
						{
							option_pointer += 1;
							continue
						}
						
						if unlikely!(option_pointer + 2 > end_of_options_pointer)
						{
							drop!(InternetProtocolVersion6TypeLengthValueOptionLengthUnderflow { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
						}
						let option_length = option_length(option_pointer);
						
						let total_length = option_pointer + 2 + option_length;
						
						if unlikely!(option_pointer + total_length > end_of_options_pointer)
						{
							drop!(InternetProtocolVersion6TypeLengthValueOptionDataUnderflow { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
						}
						
						// TODO: Validate PadN options are 0x00 padded.
						// TODO: Validate Options are not repeated.
						
						const TopTwoBits: u8 = 0b11_000000;
						match option_type && TopTwoBits
						{
							// Skip
							0b00_000000 => match option_type
							{
								QuickStartOptionType | CalipsoOptionType | SmfDpdOptionType | Experiment1EOptionType | Experiment3EOptionType => drop!(InternetProtocolVersion6TypeLengthValueOptionShouldNotBeUsedOnTheInternet { ethernet_addresses: $ethernet_addresses, header: $self, option_type }, $packet_processing, $packet),
								
								_ => (),
							},
							
							// Discard
							0b01_000000 => drop!(InternetProtocolVersion6TypeLengthValueOptionDiscardPacket { ethernet_addresses: $ethernet_addresses, header: $self, option_type }, $packet_processing, $packet),
							
							// Discard and send ICMP Parameter Problem, Code 2 to source addres.
							0b10_000000 => drop!(InternetProtocolVersion6TypeLengthValueOptionDiscardPacket { ethernet_addresses: $ethernet_addresses, header: $self, option_type }, $packet_processing, $packet),
							
							// Discard and send ICMP Parameter Problem, Code 2 to source addres only if it is multicast.
							0b11_000000 => drop!(InternetProtocolVersion6TypeLengthValueOptionDiscardPacket { ethernet_addresses: $ethernet_addresses, header: $self, option_type }, $packet_processing, $packet),
						}
						
						option_pointer += total_length
					}
					
					(next_header($extension_header_pointer), $extension_header_pointer + header_extension_length)
				}
			}
		}
		
		macro_rules! drop_bad_fragments
		{
			($self: ident, $packet: ident, $packet_processing: ident, $ethernet_addresses: ident, $fragment_extension: ident, $layer_4_length: ident) =>
			{
				{
					if let Some((_, true, fragment_offset, _fragment_identifier)) = $fragment_extension
					{
						// RFC 8200 Section 4.5: "If the length of a fragment, as derived from the fragment packet's Payload Length field, is not a multiple of 8 octets and the M flag of that fragment is 1, then that fragment must be discarded and an ICMP Parameter Problem, Code 0, message should be sent to the source of the fragment, pointing to the Payload Length field of the fragment packet.
						if $layer_4_length % 8 != 0
						{
							// We do not send an ICMP Parameter Problem, Code 0 message.
							drop!(InternetProtocolVersion6PacketFragmentNotAMultipleOf8 { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
						}
						
						// RFC 8220 Section 4.5: "If the length and offset of a fragment are such that the Payload Length of the packet reassembled from that fragment would exceed 65,535 octets, then that fragment must be discarded and an ICMP Parameter Problem, Code 0, message should be sent to the source of the fragment, pointing to the Fragment Offset field of the fragment packet".
						if (fragment_offset as usize) + $layer_4_length > 65_535
						{
							// We do not send an ICMP Parameter Problem, Code 0 message.
							drop!(InternetProtocolVersion6PacketFragmentWouldMakeReassembledPacketWouldTooLarge { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
						}
						
						#[cfg(feature = "drop-overly-small-ipv6-fragments-aggresively")] const MaximumTransmissionUnit: usize = 1280;
						#[cfg(not(feature = "drop-overly-small-ipv6-fragments-aggresively"))] const MaximumTransmissionUnit: usize = 440;
						
						const MinimalFragmentSizeForAllButFinalFragment: usize = MaximumTransmissionUnit - size_of::<InternetProtocolVersion6PacketHeader>();
						if layer_4_length < MinimalFragmentSizeForAllButFinalFragment
						{
							drop!(InternetProtocolVersion6PacketFragmentTooSmall { ethernet_addresses: $ethernet_addresses, header: $self }, $packet_processing, $packet)
						}
					}
				}
			}
		}
		
		/// RFC 8200.
		const HopByHopOptions: u8 = 0;
		
		/// Transmission Control Protocol (TCP).
		///
		/// RFC 793.
		const TransmissionControlProtocol: u8 = 6;
		
		/// User Datagram Protocol (UDP).
		///
		/// RFC 768.
		const UserDatagramProtocol: u8 = 17;
		
		/// RFC 8200.
		const Routing: u8 = 43;
		
		/// RFC 8200.
		const Fragment: u8 = 44;
		
		/// RFC 8200.
		const EncapulatingSecurityPayload: u8 = 50;
		
		/// RFC 8200.
		const AuthenticationHeader: u8 = 50;
		
		const InternetControlMessageProtocolIpV6: u8 = 58;
		
		/// RFC 8200.
		const NoNextHeader: u8 = 59;
		
		/// RFC 8200.
		const DestinationOptions: u8 = 60;
		
		/// RFC 6275.
		const Mobility: u8 = 135;
		
		/// RFC 7401.
		const HostIdentityProtocol: u8 = 139;
		
		/// RFC 5533.
		const Shim6Protocol: u8 = 140;
		
		/// RFC 4727.
		const Experimentation253: u8 = 253;
		
		/// RFC 4727.
		const Experimentation254: u8 = 254;
		
		const OctetsPerLengthUnit: u16 = 8;
		
		/// RFC 4782.
		const QuickStartOptionType: u8 = 0x26;
		
		/// RFC 5570.
		const CalipsoOptionType: u8 = 0x07;
		
		/// RFC 6621.
		const SmfDpdOptionType: u8 = 0x08;
		
		/// RFC 4727.
		const Experiment1EOptionType: u8 = 0x1E;
		
		/// RFC 4727.
		const Experiment3EOptionType: u8 = 0x3E;
		
		/// RFC 5095.
		const SourceRouteRoutingHeader: u8 = 0;
		
		const NimrodRoutingHeader: u8 = 1;
		
		/// RFC 6275.
		const Type2RoutingHeader: u8 = 2;
		
		/// RFC 6554.
		const RplSourceRouteRoutingHeader: u8 = 3;
		
		/// RFC 4727.
		const Experiment253RoutingHeader: u8 = 253;
		
		/// RFC 4727.
		const Experiment254RoutingHeader: u8 = 254;
		
		const ReservedRoutingHeader: u8 = 255;
		
		#[inline(always)]
		fn next_header(extension_header_pointer: usize) -> u8
		{
			unsafe { * (extension_header_pointer as *const u8) }
		}
		
		#[inline(always)]
		fn length_or_reserved(extension_header_pointer: usize) -> u8
		{
			unsafe { * ((extension_header_pointer + 1) as *const u8) }
		}
		
		/// RFC 8200 Section 4.3: "Length of the (extension) header in 8-octet units, not including the first 8 octets".
		#[inline(always)]
		fn extension_header_length_in_bytes(extension_header_pointer: usize) -> usize
		{
			((length_or_reserved(extension_header_pointer) as usize) * OctetsPerLengthUnit) + OctetsPerLengthUnit
		}
		
		#[inline(always)]
		fn routing_type(extension_header_pointer: usize) -> u8
		{
			unsafe { * ((extension_header_pointer + 2) as *const u8) }
		}
		
		#[inline(always)]
		fn segments_left(extension_header_pointer: usize) -> u8
		{
			unsafe { * ((extension_header_pointer + 2) as *const u8) }
		}
		
		#[inline(always)]
		fn option_type(option_pointer: usize) -> u8
		{
			unsafe { * (option_pointer as *const u8) }
		}
		
		#[inline(always)]
		fn option_length(option_pointer: usize) -> usize
		{
			(unsafe { * ((option_pointer + 1) as *const u8) }) as usize
		}
		
		#[inline(always)]
		fn fragment_offset_and_reserved_and_more_flag(extension_header_pointer: usize) -> [u8; 2]
		{
			unsafe { * ((extension_header_pointer + 2) as *const [u8; 2]) }
		}
		
		#[inline(always)]
		fn fragment_more_flag_and_offset(first_byte: u8, second_byte: u8) -> (bool, u16)
		{
			const MoreFlagBitMask: u8 = 0b1;
			
			let fragment_more_flag_set = second_byte & MoreFlagBitMask != 0;
			let fragment_offset = ((first_byte as u16) << 5 | (second_byte as u16) >> 3) * OctetsPerLengthUnit;
			(fragment_more_flag_set, fragment_offset)
		}
		
		#[inline(always)]
		fn fragment_identifier(extension_header_pointer: usize) -> [u8; 4]
		{
			unsafe { * ((extension_header_pointer + 4) as *const [u8; 4]) }
		}
		
		debug_assert!(layer_3_length >= Self::HeaderSizeU16, "Not enough layer 3 length left");
		
		let start_layer3_pointer = (self as *const Self as usize);
		let end_fixed_size_header_pointer = start_layer3_pointer + Self::HeaderSize;
		let end_layer3_pointer = end_fixed_size_header_pointer + (layer_3_length as usize);
		
		// TODO.
		xxx;
		let mut header_extensions_count = 0;
		
		
		let mut extension_header_pointer = end_fixed_size_header_pointer;
		let mut next_header = self.next_header.into();
		let mut is_after_first_extension_header = false;
		let mut routing_encountered = false;
		let mut fragment_extension: Option<(NonNull<u8>, bool, u16, [u8; 4])> = None;
		let mut destination_options_count = 0;
		loop
		{
			match next_header.into()
			{
				InternetProtocolVersion6HopByHopOptionsIsNotFirstExtensionHeader => if is_after_first_extension_header
				{
					drop!(InternetProtocolVersion6NoNextHeaderIsUnsupported { ethernet_addresses, header }, $packet_processing, $packet)
				}
				else
				{
					let (new_next_header, new_extension_header_pointer) = process_type_length_value_options!(self, packet, packet_processing, extension_header_pointer, end_layer3_pointer, ethernet_addresses);
					next_header = new_next_header;
					extension_header_pointer = new_extension_header_pointer;
				},
				
				TransmissionControlProtocol =>
				{
					if address_support.is_not_unicast()
					{
						drop!(InternetProtocolVersion6TransmissionControlProtocolPacketsShouldOnlyBeUnicast { ethernet_addresses, header }, packet_processing, packet)
					}
					
					let layer_4_length = end_layer3_pointer - extension_header_pointer;
					
					drop_bad_fragments!(self, packet, packet_processing, ethernet_addresses, fragment_extension, layer_4_length);
					
					// TODO: do fragmentation, then determine if should processing.
					
					break;
				}
				
				UserDatagramProtocol =>
				{
					let layer_4_length = end_layer3_pointer - extension_header_pointer;
					
					drop_bad_fragments!(self, packet, packet_processing, ethernet_addresses, fragment_extension, layer_4_length);
					
					// TODO: Check fragmented packets and RSS.
					// Use the identification in IPv4 and ?in IPv6? and forward the packets to a hash of all available processors, or,
					// Use a dedicated processor for fragmented packets and then re-inject.
					
					// TODO: IPv6 receivers must discard UDP packets containing a zero checksum and should log the error.
					
					let udp_hdr = unsafe { & * (end_layer3_pointer as *const udp_hdr) };
					if udp_hdr.dgram_cksum == 0x0000
					{
						drop!(InternetProtocolVersion6UserDatagramProtocolPacketsMustHaveAChecksumSet { ethernet_addresses, header }, packet_processing, packet)
					}
					
					// udp_hdr
					
					// TODO: unicast / multicast / broadcast
					todo
					break;
				}
				
				Routing =>
				{
					if routing_encountered
					{
						drop!(InternetProtocolVersion6RoutingExtensionHeaderRepeated { ethernet_addresses, header }, $packet_processing, $packet)
					}
					
					const RoutingExtensionHeaderMinimumSize: usize = OctetsPerLengthUnit;
					
					if unlikely!(extension_header_pointer + RoutingExtensionHeaderMinimumSize > end_layer3_pointer)
					{
						drop!(InternetProtocolVersion6RoutingExtensionHeaderUnderflow { ethernet_addresses, header }, packet_processing, packet)
					}
					
					let routing_type = routing_type(extension_header_pointer);
					let segments_left = segments_left(extension_header_pointer);
					
					match routing_type
					{
						SourceRouteRoutingHeader | NimrodRoutingHeader | Experiment253RoutingHeader | Experiment254RoutingHeader | ReservedRoutingHeader => drop!(InternetProtocolVersion6RoutingExtensionHeaderRoutingTypeIsDeprecatedExperimentalOrReserved { ethernet_addresses, header, routing_type, segments_left }, packet_processing, packet),
						
						_ => if unlikely!(segments_left != 0)
						{
							// We do not send an ICMP Parameter Problem, Code 0.
							drop!(InternetProtocolVersion6RoutingExtensionHeaderHasSegmentsLeft { ethernet_addresses, header, routing_type, segments_left }, packet_processing, packet)
						},
					}
					
					next_header = next_header(extension_header_pointer);
					extension_header_pointer += extension_header_length_in_bytes(extension_header_pointer);
					
					routing_encountered = true;
				}
				
				Fragment =>
				{
					if fragment_extension.is_some()
					{
						drop!(InternetProtocolVersion6FragmentExtensionHeaderRepeated { ethernet_addresses, header }, packet_processing, packet)
					}
					
					const FragmentExtensionHeaderSize: usize = 8;
					
					if unlikely!(extension_header_pointer + FragmentExtensionHeaderSize > end_layer3_pointer)
					{
						drop!(InternetProtocolVersion6FragmentExtensionHeaderUnderflow { ethernet_addresses, header }, packet_processing, packet)
					}
					
					if cfg!(feature = "drop-ipv6-fragments-when-first-reserved-field-is-not-zero")
					{
						let reserved = length_or_reserved(extension_header_pointer);
						if unlikely!(reserved != 0x00)
						{
							drop!(InternetProtocolVersion6FragmentExtensionHeaderFirstReservedFieldNonZero { ethernet_addresses, header, reserved }, packet_processing, packet)
						}
					}
					
					let fragment_details =
						{
							let fragment_offset_and_reserved_and_more_flag = fragment_offset_and_reserved_and_more_flag(extension_header_pointer);
							
							let second_byte = fragment_offset_and_reserved_and_more_flag[1];
							
							if cfg!(feature = "drop-ipv6-fragments-when-second-reserved-field-is-not-zero")
							{
								const ReservedBitsBitMask: u8 = 0b110;
								
								if unlikely!(second_byte & ReservedBitsBitMask != 0)
								{
									drop!(InternetProtocolVersion6FragmentExtensionHeaderSecondReservedFieldNonZero { ethernet_addresses, header, reserved }, packet_processing, packet)
								}
							}
							
							let (more_flag_set, fragment_offset) = fragment_more_flag_and_offset(fragment_offset_and_reserved_and_more_flag[0], second_byte);
							
							if !more_flag_set
							{
								if unlikely!(fragment_offset == 0)
								{
									drop!(InternetProtocolVersion6FragmentExtensionHeaderOnlyOneFragmentOrLastFragmentIsFirst { ethernet_addresses, header }, packet_processing, packet)
								}
							}
							
							let fragment_identifier = fragment_identifier(extension_header_pointer);
							
							(unsafe { NonNull::new_unchecked(extension_header_pointer as *mut u8) }, more_flag_set, fragment_offset, fragment_identifier)
						};
					
					fragment_extension = Some(fragment_details);
					
					next_header = next_header(extension_header_pointer);
					extension_header_pointer += FragmentExtensionHeaderSize as usize;
				}
				
				EncapulatingSecurityPayload => drop!(InternetProtocolVersion6EncapulatingSecurityPayloadExtensionHeaderUnsupported { ethernet_addresses, header }, packet_processing, packet),
				
				AuthenticationHeader => drop!(InternetProtocolVersion6AuthenticationHeaderExtensionHeaderUnsupported { ethernet_addresses, header }, packet_processing, packet),
				
				InternetControlMessageProtocolIpV6 =>
				{
					if unlikely!(fragment_extension.is_some())
					{
						drop!(InternetProtocolVersion6InternetControlMessageProtocolPacketsShouldNotBeFragmented { ethernet_addresses, header }, packet_processing, packet)
					}
					
					let layer_4_length = end_layer3_pointer - extension_header_pointer;
					
					// NOTE: ICMPv6 traffic can be received at a multicast address for 'parameter problem'.
					
					// TODO
					/// ? only allow unicast addresses ?
					
					todo
					break;
				}
				
				NoNextHeader => drop!(InternetProtocolVersion6NoNextHeaderIsUnsupported { ethernet_addresses, header }, packet_processing, packet),
				
				DestinationOptions =>
				{
					if destination_options_count == 2
					{
						drop!(InternetProtocolVersion6MoreThanTwoDestinationOptionsExtensionHeaders { ethernet_addresses, header }, packet_processing, packet)
					}
					
					let (new_next_header, new_extension_header_pointer) = process_type_length_value_options!(self, packet, packet_processing, extension_header_pointer, end_layer3_pointer, ethernet_addresses);
					next_header = new_next_header;
					extension_header_pointer = new_extension_header_pointer;
					
					destination_options_count += 1;
				}
				
				Mobility => drop!(InternetProtocolVersion6MobilityExtensionHeaderUnsupported { ethernet_addresses, header }, packet_processing, packet),
				
				HostIdentityProtocol => drop!(InternetProtocolVersion6HostIdentityProtocolExtensionHeaderUnsupported { ethernet_addresses, header }, packet_processing, packet),
				
				Shim6Protocol => drop!(InternetProtocolVersion6Shim6ProtocolExtensionHeaderUnsupported { ethernet_addresses, header }, packet_processing, packet),
				
				Experimentation253 | Experimentation254 => drop!(InternetProtocolVersion6ExperimentationExtensionHeaderUnsupported { ethernet_addresses, header }, packet_processing, packet),
				
				_ => drop!(InternetProtocolVersion6UnrecognisedExtensionHeaderOrLayer4Protocol { ethernet_addresses, header, next_header }, packet_processing, packet)
			}
			
			is_after_first_extension_header = true;
		}
		
		let packet = match packet_processing.reassemble_fragmented_internet_protocol_version_6_packet(XXXXXXXXX)
		{
			None => return,
			Some(packet) => packet,
		};
		
		let destination_address = &header.destination_address;
		
		// TODO: Source address may be 0.0.0.0 for DHCPDISCOVER broadcast; there might be something analogous in IPv6?
		xxx;
		if unlikely(source_address.is_not_valid_unicast())
		{
			drop!(InternetProtocolVersion6SourceAddressNotValidUnicast { ethernet_addresses, header }, packet_processing, packet)
		}
		
	}
}
