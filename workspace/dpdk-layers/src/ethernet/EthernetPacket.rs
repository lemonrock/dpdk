// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct EthernetPacket
{
	/// Header.
	pub header: EthernetPacketHeader,

	/// Payload
	pub payload: EthernetPacketPayload,
}

impl Display for EthernetPacket
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

macro_rules! parse_802_1q_virtual_lan_tag_control_information_drop
{
	($reason: tt, $self: ident, $tag_control_information: ident, $packet_processing: ident, $packet: ident) =>
	{
		{
			let reason = $reason
			{
				ethernet_addresses: $self.ethernet_addresses(),
				tag_control_information: $tag_control_information,
			};
		
			drop!(reason, $packet_processing, $packet)
		}
	}
}

macro_rules! parse_802_1q_virtual_lan_tag_control_information
{
	($self: ident, $tag_control_information: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		match $tag_control_information.parse()
		{
			Err(_) => parse_802_1q_virtual_lan_tag_control_information_drop!(CouldNotParse8011QVirtualLanTag, $self, $tag_control_information, $packet_processing_by_virtual_lan, $packet),

			Ok((class_of_service, drop_eligible_indicator, inner_virtual_lan_identifier)) =>
			{
				match $packet_processing_by_virtual_lan.get_packet_processing_for_inner_virtual_lan(inner_virtual_lan_identifier)
				{
					None => parse_802_1q_virtual_lan_tag_control_information_drop!(NoConfigurationFor8011QVirtualLan, $self, $tag_control_information, $packet_processing_by_virtual_lan, $packet),

					Some(packet_processing) =>
					{
						if unlikely!(packet_processing.honour_drop_eligible_indicator(drop_eligible_indicator))
						{
							parse_802_1q_virtual_lan_tag_control_information_drop!(DropEligibleFor8011QVirtualLan, $self, $tag_control_information, $packet_processing_by_virtual_lan, $packet)
						}

						if unlikely!(packet_processing.drop_packets_of_class_of_service(class_of_service))
						{
							parse_802_1q_virtual_lan_tag_control_information_drop!(DropThisClassOfServiceFor8011QVirtualLan, $self, $tag_control_information, packet_processing, $packet)
						}
						
						packet_processing
					}
				}
			}
		}
	}
}

macro_rules! parse_802_1ad_virtual_lan_tag_control_information_drop
{
	($reason: tt, $self: ident, $outer_tag_control_information: ident, $inner_tag_control_information: ident, $packet_processing: ident, $packet: ident) =>
	{
		{
			let reason = $reason
			{
				ethernet_addresses: $self.ethernet_addresses(),
				outer_tag_control_information: $outer_tag_control_information,
				inner_tag_control_information: $inner_tag_control_information,
			};
		
			drop!(reason, $packet_processing, $packet)
		}
	}
}

macro_rules! parse_802_1ad_virtual_lan_tag_control_information
{
	($self: ident, $outer_tag_control_information: ident, $inner_tag_control_information: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		{
			let (outer_virtual_lan_identifier, outer_drop_eligible_indicator, outer_class_of_service) = match $outer_tag_control_information.parse()
			{
				Err(_) => parse_802_1ad_virtual_lan_tag_control_information_drop!(CouldNotParseOuterVirtualLanTag, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet),

				Ok((class_of_service, drop_eligible_indicator, outer_virtual_lan_identifier)) =>
				{
					(outer_virtual_lan_identifier, drop_eligible_indicator, class_of_service)
				}
			};

			let (inner_virtual_lan_identifier, inner_drop_eligible_indicator, inner_class_of_service) = match $inner_tag_control_information.parse()
			{
				Err(_) => parse_802_1ad_virtual_lan_tag_control_information_drop!(CouldNotParseInnerVirtualLanTag, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet),

				Ok((class_of_service, drop_eligible_indicator, outer_virtual_lan_identifier)) =>
				{
					(outer_virtual_lan_identifier, drop_eligible_indicator, class_of_service)
				}
			};

			match $packet_processing_by_virtual_lan.get_packet_processing_for_outer_virtual_lan(outer_virtual_lan_identifier, inner_virtual_lan_identifier)
			{
				None => parse_802_1ad_virtual_lan_tag_control_information_drop!(NoConfigurationForQinQVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet),
				
				Some(packet_processing_for_q_in_q_virtual_lan) =>
				{
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.honour_outer_drop_eligible_indicator(outer_drop_eligible_indicator))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropEligibleForOuterVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.drop_packets_of_outer_class_of_service(outer_class_of_service))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropThisClassOfServiceForOuterVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.honour_inner_drop_eligible_indicator(inner_drop_eligible_indicator))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropEligibleForInnerVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.drop_packets_of_inner_class_of_service(inner_class_of_service))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropThisClassOfServiceForInnerVirtualLan, $self, $outer_tag_control_information, $outer_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					&packet_processing_for_q_in_q_virtual_lan.inner_packet_processing
				}
			}
		}
	}
}

macro_rules! process_802_1ad_virtual_lan_tagging
{
	($self: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		{
			if unlikely!($packet.is_too_short_to_be_a_qinq_vlan_ethernet_packet())
			{
				drop!(IsTooShortToBeAQinQVirtualLanEthernetPacket, $packet_processing_by_virtual_lan, $packet)
			}

			let qinq_virtual_lan_packet = $self.qinq_virtual_lan_packet();

			let outer_tag_control_information = qinq_virtual_lan_packet.tag_control_information();

			let inner_virtual_lan_packet = qinq_virtual_lan_packet.virtual_lan_packet();

			let inner_tag_control_information = inner_virtual_lan_packet.tag_control_information();

			let packet_processing = parse_802_1ad_virtual_lan_tag_control_information!($self, outer_tag_control_information, inner_tag_control_information, $packet, $packet_processing_by_virtual_lan);

			let layer_3_length = $packet.packet_length_if_contiguous_less_ethernet_packet_header() - (VirtualLanPacketHeader::IEEE_802_1ad_SizeU16 + VirtualLanPacketHeader::IEEE_802_1Q_SizeU16);
			
			let layer_3_packet = inner_virtual_lan_packet.layer_3_packet();
			
			Self::process_layer_3(layer_3_packet, $packet, packet_processing, layer_3_length, inner_virtual_lan_packet.potentially_invalid_ether_type())
		}
	}
}

macro_rules! guard_is_valid_ethernet_packet
{
	($packet_processing_by_virtual_lan: ident, $packet: ident) =>
	{
		{
			$packet.debug_assert_is_contiguous();

			if unlikely!($packet.is_too_short_to_be_an_ethernet_packet())
			{
				drop!(IsTooShortToBeAnEthernetPacket, $packet_processing_by_virtual_lan, $packet)
			}
		}
	}
}

macro_rules! guard_ethernet_addresses_drop
{
	($reason: tt, $ethernet_addresses: ident, $packet_processing: ident, $packet: ident) =>
	{
		drop!($reason { ethernet_addresses: $ethernet_addresses }, $packet_processing, $packet)
	}
}

macro_rules! guard_ethernet_addresses
{
	($self: ident, $packet: ident, $packet_processing: ident) =>
	{
		{
			let ethernet_addresses = $self.ethernet_addresses();
			let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();

			if unlikely!(source_ethernet_address.is_not_valid_unicast())
			{
				guard_ethernet_addresses_drop!(SourceEthernetAddressIsNotValidUnicast, ethernet_addresses, $packet_processing, $packet)
			}

			let we_do_not_support_sending_to_ourselves = $packet_processing.is_ethernet_address_our_valid_unicast_ethernet_address(source_ethernet_address);
			if unlikely!(we_do_not_support_sending_to_ourselves)
			{
				guard_ethernet_addresses_drop!(SourceEthernetAddressIsOurUnicastEthernetAddress, ethernet_addresses, $packet_processing, $packet)
			}

			if unlikely!($packet_processing.is_denied_source_ethernet_address(source_ethernet_address))
			{
				guard_ethernet_addresses_drop!(DeniedSourceEthernetAddress, ethernet_addresses, $packet_processing, $packet)
			}
			
			if unlikely!(destination_ethernet_address.is_zero())
			{
				guard_ethernet_addresses_drop!(DestinationEthernetAddressIsZero, ethernet_addresses, $packet_processing, $packet)
			}

			if destination_ethernet_address.is_valid_unicast()
			{
				let is_for_multiply_assigned_ethernet_addreses_on_one_link_or_promiscuous_mode_or_defective = $packet_processing.is_ethernet_address_not_our_valid_unicast_ethernet_address(destination_ethernet_address);
				if unlikely!(is_for_multiply_assigned_ethernet_addreses_on_one_link_or_promiscuous_mode_or_defective)
				{
					guard_ethernet_addresses_drop!(DestinationEthernetAddressIsNotOneOfOurs, ethernet_addresses, $packet_processing, $packet)
				}
			}

			ethernet_addresses
		}
	}
}

macro_rules! guard_ethernet_addresses_and_compute_packet_length
{
	($self: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		{
			let packet_processing = &$packet_processing_by_virtual_lan.none;

			let ethernet_addresses = guard_ethernet_addresses!($self, $packet, packet_processing);

			let layer_3_length = $packet.packet_length_if_contiguous_less_ethernet_packet_header();
			(packet_processing, layer_3_length, ethernet_addresses)
		}
	}
}

impl EthernetPacket
{
	#[inline(always)]
	pub fn process_poll_mode_driver_offloads_qinq_vlan_tagging_stripping(&mut self, packet: PacketBuffer, packet_processing_by_virtual_lan: &PacketProcessingByVirtualLan<impl PacketProcessingDropObserver>)
	{
		// TODO: Make use of packet.layer_4_hardware_packet_type() where hardware supports it - note that h/w may not support the L4_ICMP type.
		// TODO: Make use of packet.is_encapsulated_in_a_tunnel_and_has_inner_layers() where hardware supports it to get rid of packets quickly.

		guard_is_valid_ethernet_packet!(packet_processing_by_virtual_lan, packet);

		let packet_processing = if packet.was_vlan_tag_control_information_stripped()
		{
			let tag_control_information = packet.stripped_vlan_tag_control_information();
			parse_802_1q_virtual_lan_tag_control_information!(self, tag_control_information, packet, packet_processing_by_virtual_lan)
		}
		else if unlikely!(packet.was_vlan_qinq_tag_control_information_stripped())
		{
			let (outer_tag_control_information, inner_tag_control_information) = packet.stripped_vlan_qinq_tag_control_information();
			parse_802_1ad_virtual_lan_tag_control_information!(self, outer_tag_control_information, inner_tag_control_information, packet, packet_processing_by_virtual_lan)
		}
		else
		{
			&packet_processing_by_virtual_lan.none
		};
	
		packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16);
		
		let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header();
		self.process(packet, packet_processing, layer_3_length, self.potentially_invalid_ether_type())
	}

	#[inline(always)]
	pub fn process_poll_mode_driver_offloads_only_vlan_tagging_stripping(&mut self, packet: PacketBuffer, packet_processing_by_virtual_lan: &PacketProcessingByVirtualLan<impl PacketProcessingDropObserver>)
	{
		guard_is_valid_ethernet_packet!(packet_processing_by_virtual_lan, packet);

		if packet.was_vlan_tag_control_information_stripped()
		{
			let tag_control_information = packet.stripped_vlan_tag_control_information();
			let packet_processing = parse_802_1q_virtual_lan_tag_control_information!(self, tag_control_information, packet, packet_processing_by_virtual_lan);

			let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header();
			self.process(packet, packet_processing, layer_3_length, self.potentially_invalid_ether_type())
		}
		else
		{
			match self.potentially_invalid_ether_type()
			{
				EtherType::InternetProtocolVersion4 =>
				{
					packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16);
					self.process_internet_protocol_version_4(packet, packet_processing_by_virtual_lan)
				}

				EtherType::InternetProtocolVersion6 =>
				{
					packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16);
					self.process_internet_protocol_version_6(packet, packet_processing_by_virtual_lan)
				}

				EtherType::AddressResolutionProtocol =>
				{
					packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16);
					self.process_address_resolution_protocol(packet, packet_processing_by_virtual_lan)
				}

				EtherType::QinQVlanTagging =>
				{
					packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16 + VirtualLanPacketHeader::IEEE_802_1ad_SizeU16 + VirtualLanPacketHeader::IEEE_802_1Q_SizeU16);
					process_802_1ad_virtual_lan_tagging!(self, packet, packet_processing_by_virtual_lan)
				}
				
				potentially_invalid_ether_type @ _ => drop!(EthernetPacket::unsupported_ether_type(self.ethernet_addresses(), potentially_invalid_ether_type), packet_processing_by_virtual_lan, packet),
			}
		}
	}

	#[inline(always)]
	pub fn poll_mode_driver_does_not_offload_any_vlan_stripping(&mut self, packet: PacketBuffer, packet_processing_by_virtual_lan: &PacketProcessingByVirtualLan<impl PacketProcessingDropObserver>)
	{
		guard_is_valid_ethernet_packet!(packet_processing_by_virtual_lan, packet);

		match self.potentially_invalid_ether_type()
		{
			EtherType::InternetProtocolVersion4 =>
			{
				packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16);
				self.process_internet_protocol_version_4(packet, packet_processing_by_virtual_lan)
			}

			EtherType::InternetProtocolVersion6 =>
			{
				packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16);
				self.process_internet_protocol_version_6(packet, packet_processing_by_virtual_lan)
			}

			EtherType::AddressResolutionProtocol =>
			{
				packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16);
				self.process_address_resolution_protocol(packet, packet_processing_by_virtual_lan)
			}

			EtherType::QinQVlanTagging =>
			{
				packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16 + VirtualLanPacketHeader::IEEE_802_1ad_SizeU16 + VirtualLanPacketHeader::IEEE_802_1Q_SizeU16);
				process_802_1ad_virtual_lan_tagging!(self, packet, packet_processing_by_virtual_lan)
			}

			EtherType::VlanTagging =>
			{
				if unlikely!(packet.is_too_short_to_be_a_vlan_ethernet_packet())
				{
					drop!(IsTooShortToBeA8021QVirtualLanEthernetPacket, packet_processing_by_virtual_lan, packet)
				}
				
				packet.set_layer_2_header_length(EthernetPacketHeader::SizeU16 + VirtualLanPacketHeader::IEEE_802_1Q_SizeU16);

				let virtual_lan_packet = self.virtual_lan_packet();

				let tag_control_information = virtual_lan_packet.tag_control_information();
				
				let packet_processing = parse_802_1q_virtual_lan_tag_control_information!(self, tag_control_information, packet, packet_processing_by_virtual_lan);

				let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header() - VirtualLanPacketHeader::IEEE_802_1Q_SizeU16;
				
				let layer_3_packet = virtual_lan_packet.layer_3_packet();
				
				Self::process_layer_3(layer_3_packet, packet, packet_processing, layer_3_length, virtual_lan_packet.potentially_invalid_ether_type())
			}
			
			potentially_invalid_ether_type @ _ => drop!(EthernetPacket::unsupported_ether_type(self.ethernet_addresses(), potentially_invalid_ether_type), packet_processing_by_virtual_lan, packet),
		}
	}

	#[inline(always)]
	fn process(&mut self, packet: PacketBuffer, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, potentially_invalid_ether_type: EtherType)
	{
		Self::process_layer_3(self.layer_3_packet(), packet, packet_processing, layer_3_length, potentially_invalid_ether_type)
	}

	#[inline(always)]
	fn process_layer_3(layer_3_packet: &mut Layer3Packet, packet: PacketBuffer, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, potentially_invalid_ether_type: EtherType)
	{
		let ethernet_packet = packet.ethernet_packet();
		
		let ethernet_addresses = guard_ethernet_addresses!(ethernet_packet, packet, packet_processing);
		
		match potentially_invalid_ether_type
		{
			EtherType::InternetProtocolVersion4 => layer_3_packet.process_internet_protocol_version_4(packet, packet_processing, layer_3_length, ethernet_addresses),

			EtherType::InternetProtocolVersion6 => layer_3_packet.process_internet_protocol_version_6(packet, packet_processing, layer_3_length, ethernet_addresses),

			EtherType::AddressResolutionProtocol => layer_3_packet.process_address_resolution_protocol(packet, packet_processing, layer_3_length, ethernet_addresses),

			_ => drop!(EthernetPacket::unsupported_ether_type(ethernet_addresses, potentially_invalid_ether_type), packet_processing, packet),
		}
	}

	#[inline(always)]
	fn process_internet_protocol_version_4(&mut self, packet: PacketBuffer, packet_processing_by_virtual_lan: &PacketProcessingByVirtualLan<impl PacketProcessingDropObserver>)
	{
		let (packet_processing, layer_3_length, ethernet_addresses) = guard_ethernet_addresses_and_compute_packet_length!(self, packet, packet_processing_by_virtual_lan);
		self.layer_3_packet().process_internet_protocol_version_4(packet, packet_processing, layer_3_length, ethernet_addresses)
	}

	#[inline(always)]
	fn process_internet_protocol_version_6(&mut self, packet: PacketBuffer, packet_processing_by_virtual_lan: &PacketProcessingByVirtualLan<impl PacketProcessingDropObserver>)
	{
		let (packet_processing, layer_3_length, ethernet_addresses) = guard_ethernet_addresses_and_compute_packet_length!(self, packet, packet_processing_by_virtual_lan);
		self.layer_3_packet().process_internet_protocol_version_6(packet, packet_processing, layer_3_length, ethernet_addresses)
	}

	#[inline(always)]
	fn process_address_resolution_protocol(&mut self, packet: PacketBuffer, packet_processing_by_virtual_lan: &PacketProcessingByVirtualLan<impl PacketProcessingDropObserver>)
	{
		let (packet_processing, layer_3_length, ethernet_addresses) = guard_ethernet_addresses_and_compute_packet_length!(self, packet, packet_processing_by_virtual_lan);
		self.layer_3_packet().process_address_resolution_protocol(packet, packet_processing, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	fn ethernet_addresses(&self) -> &EthernetAddresses
	{
		self.header.ethernet_addresses()
	}

	#[inline(always)]
	fn potentially_invalid_ether_type(&self) -> EtherType
	{
		self.header.potentially_invalid_ether_type()
	}

	#[inline(always)]
	fn layer_3_packet(&mut self) -> &mut Layer3Packet
	{
		self.payload.layer_3_packet()
	}
	
	#[inline(always)]
	fn virtual_lan_packet(&mut self) -> &mut VirtualLanPacket
	{
		self.payload.virtual_lan_packet()
	}
	
	#[inline(always)]
	fn qinq_virtual_lan_packet(&mut self) -> &mut QinQVirtualLanPacket
	{
		self.payload.qinq_virtual_lan_packet()
	}
	
	#[inline(always)]
	fn unsupported_ether_type<'a>(ethernet_addresses: &'a EthernetAddresses, potentially_invalid_ether_type: EtherType) -> PacketProcessingDropReason<'a>
	{
		UnsupportedEtherType
		{
			ethernet_addresses,
			unsuspported_ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize
			{
				ether_type: potentially_invalid_ether_type,
			}
		}
	}
}
