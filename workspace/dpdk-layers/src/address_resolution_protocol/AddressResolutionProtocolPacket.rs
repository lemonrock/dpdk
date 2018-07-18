// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct AddressResolutionProtocolPacket
{
	/// Header.
	pub header: AddressResolutionProtocolPacketHeader,
	
	/// Payload.
	pub payload: AddressResolutionProtocolPacketPayload,
}

impl AddressResolutionProtocolPacket
{
	/// Use this to eliminate invalid traffic.
	#[inline(always)]
	pub(crate) fn is_packet_length_too_short(layer_3_length: u16) -> bool
	{
		layer_3_length < AddressResolutionProtocolPacketHeader::HeaderSizeU16
	}
	
	/// Use this to eliminate obsolete ARP traffic.
	#[inline(always)]
	pub(crate) fn is_invalid_for_internet_protocol_version_4(&self, layer_3_length: u16) -> bool
	{
		self.is_layer_3_length_invalid_for_internet_protocol_version_4(layer_3_length) || self.header.is_header_invalid_for_internet_protocol_version_4()
	}
	
	#[inline(always)]
	fn is_layer_3_length_invalid_for_internet_protocol_version_4(&self, layer_3_length: u16) -> bool
	{
		const PayloadSizeU16: u16 = size_of::<AddressResolutionProtocolPacketInternetProtocolVersion4Payload>() as u16;
		
		layer_3_length != AddressResolutionProtocolPacketHeader::HeaderSizeU16 + PayloadSizeU16
	}
	
	#[inline(always)]
	pub(crate) fn process(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
	{
		debug_assert!(source_ethernet_address.is_valid_unicast(), "source_ethernet_address '{}' is not valid unicast", source_ethernet_address);
		
		if unlikely!(destination_ethernet_address.is_multicast())
		{
			finish!(packet)
		}
		
		debug_assert!(destination_ethernet_address.is_valid_unicast() || destination_ethernet_address.is_broadcast(), "destination_ethernet_address '{}' is not valid unicast or broadcast()", destination_ethernet_address);
		
		match self.header.operation
		{
			Operation::Request => self.process_request(packet, packet_processing_configuration, source_ethernet_address, destination_ethernet_address),
			
			Operation::Reply => self.process_reply(packet, packet_processing_configuration, source_ethernet_address, destination_ethernet_address),
			
			_ => finish!(packet),
		}
	}
	
	#[inline(always)]
	fn process_request(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
	{
		// Is the destination ethernet address invalid for an Address Resolution Protocol (ARP) request?
		//
		// Aside from RFC 1122 § 2.3.2.1, which is a minor feature to re-validate cached ARP entries, there is no good reason to receive unicast (or indeed multicast, or anything other than broadcast) ARP requests.
		// See first answer at [StackOverflow|https://security.stackexchange.com/questions/58131/unicast-arp-requests-considered-harmful] for a longer discussion.
		// Consequently we consider anything other than ARP requests with a broadcast as invalid.
		//
		// TODO: Note, however, that not supporting this caused a problem for Mac OS Mavericks: <https://www.reddit.com/r/sysadmin/comments/1yc6n1/packet_losses_with_new_os_x_mavericks_make_sure/>.
		if destination_ethernet_address.is_not_broadcast()
		{
			finish!(packet)
		}
		
		let payload = self.payload();
		
//		Strictly speaking, RFC 5227 makes this a SHOULD not a MUST.
//		if unlikely!(payload.target_hardware_address.is_not_zero())
//		{
//			finish!(packet)
//		}
		
		let sender_hardware_address = &payload.sender_hardware_address;
		if unlikely!(source_ethernet_address != sender_hardware_address)
		{
			finish!(packet)
		}
		
		let target_protocol_address = payload.target_protocol_address;
		if unlikely!(target_protocol_address.is_not_valid_unicast())
		{
			finish!(packet)
		}
		
		let sender_protocol_address = payload.sender_protocol_address;
		
		// sender_hardware_address: MUST be valid source ethernet address.
		// sender_protocol_address: MUST be all zeros (unspecified).
		// target_hardware_address: SHOULD be zeros; it is ignored.
		// target_protocol_address: MUST be set to address being probed.
		let is_arp_probe = sender_protocol_address.is_unspecified();
		if is_arp_probe
		{
			let we_own_the_target_protocol_address_so_reply = packet_processing_configuration.is_internet_protocol_version_4_host_address_one_of_ours(target_protocol_address);
			if unlikely!(we_own_the_target_protocol_address_so_reply)
			{
				// TODO: REPLY
				// Mutate the ethernet packet and arp packet, then add to an outbound queue.
				eprintln!("ARP is not supported");
				finish!(packet)
			}
			else
			{
				finish!(packet)
			}
		}
		else
		{
			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				finish!(packet)
			}
			
			let internet_protocol_version_4_host_address_conflict = packet_processing_configuration.is_internet_protocol_version_4_host_address_one_of_ours(sender_protocol_address);
			if internet_protocol_version_4_host_address_conflict
			{
				packet_processing_configuration.internet_protocol_version_4_host_address_conflict(packet);
				return
			}
			
			// Also known as a gratuitous ARP request.
			let is_arp_announcement = sender_protocol_address == target_protocol_address;
			if is_arp_announcement
			{
				packet_processing_configuration.add_to_address_resolution_cache(sender_hardware_address, sender_protocol_address);
				finish!(packet)
			}
			else
			{
				let we_own_the_target_protocol_address_so_reply = packet_processing_configuration.is_internet_protocol_version_4_host_address_one_of_ours(target_protocol_address);
				if we_own_the_target_protocol_address_so_reply
				{
					// TODO: REPLY
					eprintln!("ARP is not supported");
					finish!(packet)
				}
			}
		}
	}
	
	#[inline(always)]
	fn process_reply(&mut self, packet: PacketBuffer, packet_processing_configuration: &PacketProcessingConfiguration, source_ethernet_address: &MediaAccessControlAddress, destination_ethernet_address: &MediaAccessControlAddress)
	{
		let payload = self.payload();
		let sender_hardware_address = &payload.sender_hardware_address;
		let target_hardware_address = &payload.target_hardware_address;
		let sender_protocol_address = payload.sender_protocol_address;
		let target_protocol_address = payload.target_protocol_address;
		
		let internet_protocol_version_4_host_address_conflict = packet_processing_configuration.is_internet_protocol_version_4_host_address_one_of_ours(sender_protocol_address);
		if internet_protocol_version_4_host_address_conflict
		{
			packet_processing_configuration.internet_protocol_version_4_host_address_conflict(packet);
			return
		}
		
		let sender_and_target_protocol_addresses_are_the_same = sender_protocol_address == target_protocol_address;
		
		// A gratuitous ARP reply is a reply to which no request has been made.
		// These are less common than a gratuitous ARP request, and not preferred, see RFC 5227 Section 3.
		let is_gratuitous_arp_reply = sender_and_target_protocol_addresses_are_the_same && (target_hardware_address.is_broadcast() || target_hardware_address.is_zero());
		if is_gratuitous_arp_reply
		{
			let protocol_address = sender_protocol_address;
			
			if unlikely!(protocol_address.is_not_valid_unicast())
			{
				finish!(packet)
			}
			
			packet_processing_configuration.add_to_address_resolution_cache(sender_hardware_address, protocol_address);
		}
		else
		{
			if unlikely!(source_ethernet_address != sender_hardware_address)
			{
				finish!(packet)
			}
			
			if unlikely!(destination_ethernet_address != target_hardware_address)
			{
				finish!(packet)
			}
			
			if unlikely!(target_hardware_address.is_not_valid_unicast())
			{
				finish!(packet)
			}
			
			if unlikely!(sender_and_target_protocol_addresses_are_the_same)
			{
				finish!(packet)
			}
			
			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				finish!(packet)
			}
			
			if unlikely!(target_protocol_address.is_not_valid_unicast())
			{
				finish!(packet)
			}
			
			packet_processing_configuration.add_to_address_resolution_cache(sender_hardware_address, sender_protocol_address);
		}
		
		finish!(packet)
	}
	
	#[inline(always)]
	fn payload(&mut self) -> &mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload
	{
		unsafe { &mut self.payload.internet_protocol_version_4_payload }
	}
}
