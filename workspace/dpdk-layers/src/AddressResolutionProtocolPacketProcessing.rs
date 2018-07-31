// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension trait for an Address Resolution Protocol (ARP) packet.
pub trait AddressResolutionProtocolPacketProcessing
{
	/// Process.
	#[inline(always)]
	fn process<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, ethernet_addresses: &'a EthernetAddresses);
	
	#[doc(hidden)]
	#[inline(always)]
	fn process_request<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, ethernet_addresses: &'a EthernetAddresses);
	
	#[doc(hidden)]
	#[inline(always)]
	fn process_reply<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, ethernet_addresses: &'a EthernetAddresses);
	
	#[doc(hidden)]
	#[inline(always)]
	fn internet_protocol_version_4_host_address_conflict(&self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>);
}

impl AddressResolutionProtocolPacketProcessing for AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn process<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, ethernet_addresses: &'a EthernetAddresses)
	{
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		
		debug_assert!(source_ethernet_address.is_valid_unicast(), "source_ethernet_address '{}' is not valid unicast", source_ethernet_address);

		if unlikely!(destination_ethernet_address.is_multicast())
		{
			drop!(AddressResolutionProtocolDestinationEthernetAddressIsMulticast { ethernet_addresses, header: &self.header }, packet_processing, packet)
		}

		debug_assert!(destination_ethernet_address.is_valid_unicast() || destination_ethernet_address.is_broadcast(), "destination_ethernet_address '{}' is not valid unicast or broadcast()", destination_ethernet_address);

		match self.header.operation
		{
			Operation::Request => self.process_request(packet, packet_processing, ethernet_addresses),

			Operation::Reply => self.process_reply(packet, packet_processing, ethernet_addresses),

			_ => drop!(AddressResolutionProtocolOperationIsUnsupported { ethernet_addresses, header: &self.header }, packet_processing, packet),
		}
	}

	#[inline(always)]
	fn process_request<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, ethernet_addresses: &'a EthernetAddresses)
	{
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		
		// RFC 1122 Section 2.3.2.1: "(2) Unicast Poll -- Actively poll the remote host by periodically sending a point-to-point ARP Request to it, and delete the entry if no ARP Reply is received from N successive polls".
		//
		// Thus an ARP request should be either to a broadcast address (normal behaviour) or to an unicast address.
		if destination_ethernet_address.is_multicast()
		{
			drop!(AddressResolutionProtocolRequestIsMulticast { ethernet_addresses, header: &self.header }, packet_processing, packet)
		}

		let payload = self.payload();

		// This violates RFC 5227 which states that requests SHOULD have a non-zero target hardware address.
		if cfg!(feature = "drop-arp-requests-with-non-zero-target-hardware-address")
		{
			if unlikely!(payload.target_hardware_address.is_not_zero())
			{
				drop!(AddressResolutionProtocolRequestTargetHardwareAddressIsZero { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}
		}

		let sender_hardware_address = &payload.sender_hardware_address;
		if unlikely!(source_ethernet_address != sender_hardware_address)
		{
			drop!(AddressResolutionProtocolHardwareAndPacketSourceEthernetAddressMismatch { ethernet_addresses, header: &self.header }, packet_processing, packet)
		}

		let target_protocol_address = payload.target_protocol_address;
		if unlikely!(target_protocol_address.is_not_valid_unicast())
		{
			drop!(AddressResolutionProtocolHardwareAndPacketDestinationEthernetAddressMismatch { ethernet_addresses, header: &self.header }, packet_processing, packet)
		}

		let sender_protocol_address = payload.sender_protocol_address;

		// sender_hardware_address: MUST be valid source ethernet address.
		// sender_protocol_address: MUST be all zeros (unspecified).
		// target_hardware_address: SHOULD be zeros; it is ignored.
		// target_protocol_address: MUST be set to address being probed.
		let is_arp_probe = sender_protocol_address.is_unspecified();
		if is_arp_probe
		{
			let we_own_the_target_protocol_address_so_reply = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(target_protocol_address);
			if unlikely!(we_own_the_target_protocol_address_so_reply)
			{
				// TODO: REPLY
				// Mutate the ethernet packet and arp packet, then add to an outbound queue.
				unsupported!("ARP replies to probes are not supported");
				drop!(ReuseInReply, packet_processing, packet)
			}
			else
			{
				drop!(AddressResolutionProtocolProbeIsNotForUs { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}
		}
		else
		{
			if destination_ethernet_address.is_not_broadcast()
			{
				drop!(AddressResolutionProtocolRequestIsNotAProbeAndIsNotBroadcast { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}

			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				drop!(AddressResolutionProtocolRequestIsNotAProbeAndSenderProtocolAddressIsNotUnicast { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}

			let internet_protocol_version_4_host_address_conflict = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(sender_protocol_address);
			if internet_protocol_version_4_host_address_conflict
			{
				self.internet_protocol_version_4_host_address_conflict(packet, packet_processing);
				return
			}

			// Also known as a gratuitous ARP request.
			let is_arp_announcement = sender_protocol_address == target_protocol_address;
			if is_arp_announcement
			{
				packet_processing.add_to_address_resolution_cache(sender_hardware_address, sender_protocol_address, packet);
				return
			}

			let we_own_the_target_protocol_address_so_reply = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(target_protocol_address);
			if we_own_the_target_protocol_address_so_reply
			{
				// TODO: REPLY
				unsupported!("ARP replies to broadcasts are not supported");
				drop!(ReuseInReply, packet_processing, packet)
			}

			packet.free_direct_contiguous_packet();
			return
		}
	}

	#[inline(always)]
	fn process_reply<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, ethernet_addresses: &'a EthernetAddresses)
	{
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		
		let payload = self.payload();
		let sender_hardware_address = &payload.sender_hardware_address;
		let target_hardware_address = &payload.target_hardware_address;
		let sender_protocol_address = payload.sender_protocol_address;
		let target_protocol_address = payload.target_protocol_address;

		let internet_protocol_version_4_host_address_conflict = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(sender_protocol_address);
		if internet_protocol_version_4_host_address_conflict
		{
			self.internet_protocol_version_4_host_address_conflict(packet, packet_processing);
			return
		}

		let sender_and_target_protocol_addresses_are_the_same = sender_protocol_address == target_protocol_address;

		// A gratuitous ARP reply is a reply to which no request has been made.
		// These are less common than a gratuitous ARP request, and not preferred, see RFC 5227 Section 3.
		let is_gratuitous_arp_reply = sender_and_target_protocol_addresses_are_the_same && (target_hardware_address.is_broadcast() || target_hardware_address.is_zero());
		if is_gratuitous_arp_reply
		{
			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				drop!(AddressResolutionProtocolGratuitousReplyIsNotValidUnicast { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}
		}
		else
		{
			if unlikely!(source_ethernet_address != sender_hardware_address)
			{
				drop!(AddressResolutionProtocolHardwareAndPacketSourceEthernetAddressMismatch { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}

			if unlikely!(destination_ethernet_address != target_hardware_address)
			{
				drop!(AddressResolutionProtocolHardwareAndPacketDestinationEthernetAddressMismatch { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}

			if unlikely!(target_hardware_address.is_not_valid_unicast())
			{
				drop!(AddressResolutionProtocolReplyTargetHardwareAddressIsNotValidUnicast { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}

			if unlikely!(sender_and_target_protocol_addresses_are_the_same)
			{
				drop!(AddressResolutionProtocolReplySourceAndTargetProtocolAddressesAreTheSame { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}

			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				drop!(AddressResolutionProtocolReplySenderProtocolAddressIsNotValidUnicast { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}

			if unlikely!(target_protocol_address.is_not_valid_unicast())
			{
				drop!(AddressResolutionProtocolReplyTargetProtocolAddressIsNotValidUnicast { ethernet_addresses, header: &self.header }, packet_processing, packet)
			}
		}

		packet_processing.add_to_address_resolution_cache(sender_hardware_address, sender_protocol_address, packet);
	}
	
	#[inline(always)]
	fn internet_protocol_version_4_host_address_conflict(&self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>)
	{
		// TODO: Handle ARP host address conflicts; see AddressResolutionProtocolAddressConflictState.rs.
		unsupported!("ARP: host address conflict");
		drop!(ReuseInReply, self, packet)
	}
}
