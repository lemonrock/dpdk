// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct EthernetPacket
{
	/// Header.
	pub header: EthernetPacketHeader,

	/// Payload
	pub payload: EthernetPacketPayload,
}

macro_rules! parse_802_1q_virtual_lan_tag_control_information
{
	($tag_control_information: ident, $packet: ident, $packet_processing_configuration_by_virtual_lan: ident) =>
	{
		match $tag_control_information.parse()
		{
			Err(_) => finish!($packet),
			Ok((class_of_service, drop_eligible_indicator, inner_virtual_lan_identifier)) =>
			{
				if unlikely!(drop_eligible_indicator)
				{
					finish!($packet)
				}

				match $packet_processing_configuration_by_virtual_lan.get_packet_processing_for_inner_virtual_lan(inner_virtual_lan_identifier)
				{
					None => finish!($packet),
					Some(packet_processing_configuration) =>
					{
						if unlikely!(packet_processing_configuration.drop_packets_of_class_of_service(class_of_service))
						{
							finish!($packet)
						}
						packet_processing_configuration
					}
				}
			}
		}
	}
}

macro_rules! parse_802_1ad_virtual_lan_tag_control_information
{
	($outer_tag_control_information: ident, $inner_tag_control_information: ident, $packet: ident, $packet_processing_configuration_by_virtual_lan: ident) =>
	{
		{
			let (outer_virtual_lan_identifier, outer_class_of_service) = match $outer_tag_control_information.parse()
			{
				Err(_) => finish!($packet),
				Ok((class_of_service, drop_eligible_indicator, outer_virtual_lan_identifier)) =>
				{
					if unlikely!(drop_eligible_indicator)
					{
						finish!($packet)
					}

					(outer_virtual_lan_identifier, class_of_service)
				}
			};

			let (inner_virtual_lan_identifier, inner_class_of_service) = match $inner_tag_control_information.parse()
			{
				Err(_) => finish!($packet),
				Ok((class_of_service, drop_eligible_indicator, outer_virtual_lan_identifier)) =>
				{
					if unlikely!(drop_eligible_indicator)
					{
						finish!($packet)
					}

					(outer_virtual_lan_identifier, class_of_service)
				}
			};

			match $packet_processing_configuration_by_virtual_lan.get_packet_processing_for_outer_virtual_lan(outer_virtual_lan_identifier, inner_virtual_lan_identifier)
			{
				None => finish!($packet),
				Some(packet_processing) =>
				{
					if unlikely!(packet_processing.drop_packets_of_class_of_service(outer_class_of_service, inner_class_of_service))
					{
						finish!($packet)
					}
					&packet_processing.inner_packet_processing_configuration
				}
			}
		}
	}
}

macro_rules! process_802_1ad_virtual_lan_tagging
{
	($self: ident, $packet: ident, $packet_processing_configuration_by_virtual_lan: ident) =>
	{
		{
			if unlikely!($packet.is_too_short_to_be_a_qinq_vlan_ethernet_packet())
			{
				finish!($packet)
			}

			let qinq_virtual_lan_packet = unsafe { &mut $self.payload.qinq_virtual_lan_packet };

			let outer_tag_control_information = qinq_virtual_lan_packet.header.tag_control_information;

			let inner_virtual_lan_packet = unsafe { &mut qinq_virtual_lan_packet.virtual_lan_packet };

			let inner_tag_control_information = inner_virtual_lan_packet.header.tag_control_information;

			let packet_processing_configuration = parse_802_1ad_virtual_lan_tag_control_information!(outer_tag_control_information, inner_tag_control_information, $packet, $packet_processing_configuration_by_virtual_lan);

			let layer_3_length = $packet.packet_length_if_contiguous_less_ethernet_packet_header() - (VirtualLanPacketHeader::QinQVirtualLanPacketHeaderSizeU16 + VirtualLanPacketHeader::VirtualLanPacketHeaderSizeU16);
			let layer_3_packet = unsafe { &mut inner_virtual_lan_packet.layer_3_packet };
			Self::process_layer_3(layer_3_packet, $packet, packet_processing_configuration, layer_3_length, inner_virtual_lan_packet.header.potentially_invalid_ether_type())
		}
	}
}

macro_rules! guard_is_valid_ethernet_packet
{
	($packet: ident) =>
	{
		{
			$packet.debug_assert_is_contiguous();

			if unlikely!($packet.is_too_short_to_be_an_ethernet_packet())
			{
				finish!($packet)
			}
		}
	}
}

macro_rules! guard_ethernet_addresses
{
	($self: ident, $packet: ident, $packet_processing_configuration: ident) =>
	{
		{
			let source_ethernet_address = &$self.header.source_address;
			let destination_ethernet_address = &$self.header.destination_address;
			
			if unlikely!(source_ethernet_address.is_not_valid_unicast())
			{
				finish!($packet)
			}

			let we_do_not_support_sending_to_ourselves = $packet_processing_configuration.is_ethernet_address_our_valid_unicast_ethernet_address(source_ethernet_address);
			if unlikely!(we_do_not_support_sending_to_ourselves)
			{
				finish!($packet)
			}

			if unlikely!($packet_processing_configuration.is_denied_source_ethernet_address(source_ethernet_address))
			{
				finish!($packet)
			}

			if unlikely!(destination_ethernet_address.is_zero())
			{
				finish!($packet)
			}

			if destination_ethernet_address.is_valid_unicast()
			{
				let is_for_multiply_assigned_ethernet_addreses_on_one_link_or_promiscuous_mode_or_defective = $packet_processing_configuration.is_ethernet_address_not_our_valid_unicast_ethernet_address(destination_ethernet_address);
				if unlikely!(is_for_multiply_assigned_ethernet_addreses_on_one_link_or_promiscuous_mode_or_defective)
				{
					finish!($packet)
				}
			}

			(source_ethernet_address, destination_ethernet_address)
		}
	}
}

macro_rules! guard_ethernet_addresses_and_compute_packet_length
{
	($self: ident, $packet: ident, $packet_processing_configuration_by_virtual_lan: ident) =>
	{
		{
			let packet_processing_configuration = &$packet_processing_configuration_by_virtual_lan.none;

			let (source_ethernet_address, destination_ethernet_address) = guard_ethernet_addresses!($self, $packet, packet_processing_configuration);

			let layer_3_length = $packet.packet_length_if_contiguous_less_ethernet_packet_header();
			(packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address)
		}
	}
}

impl EthernetPacket
{
	#[inline(always)]
	pub fn process_poll_mode_driver_offloads_qinq_vlan_tagging_stripping(&mut self, packet: PacketBuffer, packet_processing_configuration_by_virtual_lan: &PacketProcessingConfigurationByVirtualLan)
	{
		// TODO: Make use of packet.layer_4_hardware_packet_type() where hardware supports it - note that h/w may not support the L4_ICMP type.
		// TODO: Make use of packet.is_encapsulated_in_a_tunnel_and_has_inner_layers() where hardware supports it to get rid of packets quickly.

		guard_is_valid_ethernet_packet!(packet);

		let packet_processing_configuration = if packet.was_vlan_tag_control_information_stripped()
		{
			let tag_control_information = packet.stripped_vlan_tag_control_information();
			parse_802_1q_virtual_lan_tag_control_information!(tag_control_information, packet, packet_processing_configuration_by_virtual_lan)
		}
		else if unlikely!(packet.was_vlan_qinq_tag_control_information_stripped())
		{
			let (outer_tag_control_information, inner_tag_control_information) = packet.stripped_vlan_qinq_tag_control_information();
			parse_802_1ad_virtual_lan_tag_control_information!(outer_tag_control_information, inner_tag_control_information, packet, packet_processing_configuration_by_virtual_lan)
		}
		else
		{
			&packet_processing_configuration_by_virtual_lan.none
		};

		let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header();
		self.process(packet, packet_processing_configuration, layer_3_length, self.potentially_invalid_ether_type())
	}

	#[inline(always)]
	pub fn process_poll_mode_driver_offloads_only_vlan_tagging_stripping(&mut self, packet: PacketBuffer, packet_processing_configuration_by_virtual_lan: &PacketProcessingConfigurationByVirtualLan)
	{
		guard_is_valid_ethernet_packet!(packet);

		if packet.was_vlan_tag_control_information_stripped()
		{
			let tag_control_information = packet.stripped_vlan_tag_control_information();
			let packet_processing_configuration = parse_802_1q_virtual_lan_tag_control_information!(tag_control_information, packet, packet_processing_configuration_by_virtual_lan);

			let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header();
			self.process(packet, packet_processing_configuration, layer_3_length, self.potentially_invalid_ether_type())
		}
		else
		{
			match self.potentially_invalid_ether_type()
			{
				EtherType::InternetProtocolVersion4 => self.process_internet_protocol_version_4(packet, packet_processing_configuration_by_virtual_lan),

				EtherType::InternetProtocolVersion6 => self.process_internet_protocol_version_6(packet, packet_processing_configuration_by_virtual_lan),

				EtherType::AddressResolutionProtocol => self.process_address_resolution_protocol(packet, packet_processing_configuration_by_virtual_lan),

				EtherType::QinQVlanTagging => process_802_1ad_virtual_lan_tagging!(self, packet, packet_processing_configuration_by_virtual_lan),

				_ => finish!(packet),
			}
		}
	}

	#[inline(always)]
	pub fn poll_mode_driver_does_not_offload_any_vlan_stripping(&mut self, packet: PacketBuffer, packet_processing_configuration_by_virtual_lan: &PacketProcessingConfigurationByVirtualLan)
	{
		guard_is_valid_ethernet_packet!(packet);

		match self.potentially_invalid_ether_type()
		{
			EtherType::InternetProtocolVersion4 => self.process_internet_protocol_version_4(packet, packet_processing_configuration_by_virtual_lan),

			EtherType::InternetProtocolVersion6 => self.process_internet_protocol_version_6(packet, packet_processing_configuration_by_virtual_lan),

			EtherType::AddressResolutionProtocol => self.process_address_resolution_protocol(packet, packet_processing_configuration_by_virtual_lan),

			EtherType::QinQVlanTagging => process_802_1ad_virtual_lan_tagging!(self, packet, packet_processing_configuration_by_virtual_lan),

			EtherType::VlanTagging =>
			{
				if unlikely!(packet.is_too_short_to_be_a_vlan_ethernet_packet())
				{
					finish!(packet)
				}

				let virtual_lan_packet = unsafe { &mut self.payload.virtual_lan_packet };

				let tag_control_information = virtual_lan_packet.header.tag_control_information;
				let packet_processing_configuration = parse_802_1q_virtual_lan_tag_control_information!(tag_control_information, packet, packet_processing_configuration_by_virtual_lan);

				let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header() - VirtualLanPacketHeader::VirtualLanPacketHeaderSizeU16;
				let layer_3_packet = unsafe { &mut virtual_lan_packet.layer_3_packet };
				Self::process_layer_3(layer_3_packet, packet, packet_processing_configuration, layer_3_length, virtual_lan_packet.header.potentially_invalid_ether_type())
			}

			_ => packet.free_direct_contiguous_packet(),
		}
	}

	#[inline(always)]
	fn process(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, layer_3_length: u16, potentially_invalid_ether_type: EtherType)
	{
		Self::process_layer_3(self.layer_3_packet(), packet, packet_processing_configuration, layer_3_length, potentially_invalid_ether_type)
	}

	#[inline(always)]
	fn process_layer_3(layer_3_packet: &mut Layer3Packet, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, layer_3_length: u16, potentially_invalid_ether_type: EtherType)
	{
		let ethernet_packet = packet.ethernet_packet();
		
		match potentially_invalid_ether_type
		{
			EtherType::InternetProtocolVersion4 =>
			{
				let (source_ethernet_address, destination_ethernet_address) = guard_ethernet_addresses!(ethernet_packet, packet, packet_processing_configuration);
				layer_3_packet.process_internet_protocol_version_4(packet, packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address)
			}

			EtherType::InternetProtocolVersion6 =>
			{
				let (source_ethernet_address, destination_ethernet_address) = guard_ethernet_addresses!(ethernet_packet, packet, packet_processing_configuration);
				layer_3_packet.process_internet_protocol_version_6(packet, packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address)
			}

			EtherType::AddressResolutionProtocol =>
			{
				let (source_ethernet_address, destination_ethernet_address) = guard_ethernet_addresses!(ethernet_packet, packet, packet_processing_configuration);
				layer_3_packet.process_address_resolution_protocol(packet, packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address)
			}

			_ => finish!(packet),
		}
	}

	#[inline(always)]
	fn process_internet_protocol_version_4(&mut self, packet: PacketBuffer, packet_processing_configuration_by_virtual_lan: &PacketProcessingConfigurationByVirtualLan)
	{
		let (packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address) = guard_ethernet_addresses_and_compute_packet_length!(self, packet, packet_processing_configuration_by_virtual_lan);
		self.layer_3_packet().process_internet_protocol_version_4(packet, packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address)
	}

	#[inline(always)]
	fn process_internet_protocol_version_6(&mut self, packet: PacketBuffer, packet_processing_configuration_by_virtual_lan: &PacketProcessingConfigurationByVirtualLan)
	{
		let (packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address) = guard_ethernet_addresses_and_compute_packet_length!(self, packet, packet_processing_configuration_by_virtual_lan);
		self.layer_3_packet().process_internet_protocol_version_6(packet, packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address)
	}

	#[inline(always)]
	fn process_address_resolution_protocol(&mut self, packet: PacketBuffer, _packet_processing_configuration_by_virtual_lan: &PacketProcessingConfigurationByVirtualLan)
	{
//		let (packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address) = guard_ethernet_addresses_and_compute_packet_length!(self, packet, packet_processing_configuration_by_virtual_lan);
//		self.layer_3_packet().process_address_resolution_protocol(packet, packet_processing_configuration, layer_3_length, source_ethernet_address, destination_ethernet_address)

		// At this point in time, we do not support ARP as (a) it is insecure and (b) can not communicate the maximum ethernet frame length supported by the destination.
		// If DPDK develops support for MACsec, then we may make use of it.

		finish!(packet);
	}

	#[inline(always)]
	fn potentially_invalid_ether_type(&self) -> EtherType
	{
		unsafe { self.header.ether_type_or_legacy_ethernet_frame_size.ether_type }
	}

	#[inline(always)]
	fn layer_3_packet(&mut self) -> &mut Layer3Packet
	{
		unsafe { &mut self.payload.layer_3_packet }
	}
}
