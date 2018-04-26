// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct IpV4State
{
	pub sourceIpV4AddressBlackList: IpV4AddressBlackList,
	pub arpCache: ArpCache,
	pub ourIpV4Addresses: HashMap<InternetProtocolVersion4HostAddress, IpAddressInformation>,
	pub packetReassemblyTable: InternetProtocolPacketReassemblyTable,
}

impl IpV4State
{
	#[inline(always)]
	pub fn ipV4Packet(&mut self, packet: *mut rte_mbuf, layer2HeaderLength: u32)
	{
		let destinationEthernetAddress = destinationEthernetAddress!(packet);
		discardPacketIf!(packet, MediaAccessControlAddress::ethernetAddressIsInvalid(destinationEthernetAddress));

		let (layer3Length, layer3HeaderLength, layer4Protocol, layer4HeaderLength) = mutatePacketData!(packet, layer2HeaderLength, ipv4_hdr, ipV4Header,
		{
			let mut ipV4HeaderData = unsafe { *ipV4Header };

			// Note:
			// - We ignore IPv4 options
			// - We ignore DSCP information
			// - We discard packets with non-zero ECN bits
			// - We check the checksum as late as possible; it adds very little value (it is nearly always valid) and is expensive to compute if not supported by the poll mode driver's hardware

			// Drop if not a version 4 packet
			const Top4BitsMask: u8 = 0xF0;
			const Version4BitShifted: u8 = 4 << 4;
			let versionAndInternetHeaderLength = ipV4HeaderData.version_ihl;
			discardPacketIf!(packet, versionAndInternetHeaderLength & Top4BitsMask != Version4BitShifted);

			// Drop if Internet Header Length is less than minimum
			const Bottom4BitsMask: u8 = 0x0F;
			const MinimumInternetHeaderLength: u8 = 5;
			let internetHeaderLength = versionAndInternetHeaderLength & Bottom4BitsMask;
			discardPacketIf!(packet, internetHeaderLength < MinimumInternetHeaderLength);

			// Drop if total length is less than minimum
			const MinimumTotalLength: u16 = 20;
			let totalLength = u16::from_be(ipV4HeaderData.total_length);
			discardPacketIf!(packet, totalLength < MinimumTotalLength);

			// Drop if Internet Header Length exceeds total length
			let layer3HeaderLength = internetHeaderLength * 4;
			discardPacketIf!(packet, layer3HeaderLength > totalLength);

			// Drop if Internet Header Length exceeds 255 bytes (the maximum supported by DPDK)
			discardPacketIf!(packet, layer3HeaderLength > IpState::DpdkMaximumLayer3HeaderLength);

			// Drop unsupported layer 4 protocols here and now, including ICMPv4
			let nextProtocolId = ipV4Header.next_proto_id;
			let (layer4Protocol, layer4HeaderLength) = if likely(nextProtocolId == Layer4Protocol::Tcp as u8)
			{
				fixUpTcpForTldk!(packet, layer2HeaderLength, layer3HeaderLength, RTE_PTYPE_L3_IPV4)
			}
			else if likely(nextProtocolId == Layer4Protocol::Udp as u8)
			{
				fixUpUdpForTldk!(packet, layer2HeaderLength, layer3HeaderLength, RTE_PTYPE_L3_IPV4)
			}
			else
			{
				discardPacket!(packet);
			};

			// We discard packets with ECN settings as TLDK has no support for them at this time. Might not be entirely appropriate for TCP (note that TCP control packets should not have ECN markings, but others can)
			const Bottom2BitsMask: u8 = 0b00000011;
			let explicitCongestionNotification = unsafe { transmute(ipV4HeaderData.type_of_service & Bottom2BitsMask) };
			dropPacketIf!(packet, explicitCongestionNotification != ExplicitCongestionNotification::NotCapableTransport);

			// Fragmentation checks
			let flagsAndFragmentOffset = u16::from_be(ipV4HeaderData.fragment_offset);

			const ReservedFlagBit:      u16 = 0b10000000_00000000;
			discardPacketIf!(packet, flagsAndFragmentOffset & ReservedFlagBit == ReservedFlagBit);

			const DontFragmentFlagBit:  u16 = 0b01000000_00000000;
			let dontFragment = flagsAndFragmentOffset & DontFragmentFlagBit == DontFragmentFlagBit;

			const MoreFragmentsFlagBit: u16 = 0b00100000_00000000;
			let moreFragments = flagsAndFragmentOffset & MoreFragmentsFlagBit == MoreFragmentsFlagBit;

			discardPacketIf!(packet, dontFragment & moreFragments);
			let fragmentOffset = flagsAndFragmentOffset & 0b00011111_11111111;

			discardPacketIf!(packet, dontFragment && fragmentOffset != 0);

			// Drop if source IP addresses aren't valid
			let senderIpV4Address = u32::from_be(ipV4HeaderData.src_addr);
			discardPacketIf!(packet, senderIpV4Address.is_not_valid_unicast());
			discardPacketIf!(packet, self.isBlackListedSourceIpV4Address(senderIpV4Address));
			discardPacketIf!(packet, self.isOneOfOurIpV4Addresses(senderIpV4Address));

			// Drop if the the checksum is invalid
			discardPacketIfInvalidIpV4CheckSum!(packet, ipV4Header, ipV4HeaderData);

			let destinationIpV4Address = u32::from_be(ipV4HeaderData.dst_addr);
			if moreFragments || fragmentOffset > 0
			{
				// Drop if destination IP addresses aren't valid
				discardPacketIf!(packet, self.isNotOneOfOurIpV4Addresses(destinationIpV4Address, destinationEthernetAddress));

				// free death row if full before entry

			}
			else
			{
				// TODO: Additional dest Ether test

				let internet_protocol_addressInformation = discardPacketIfNone!(packet, self.ipV4State.ourIpV4Addresses(destinationIpV4Address)).unwrap();

			}

			(false, (layer3Length, layer3HeaderLength, layer4Protocol, layer4HeaderLength))
		});

		IpState::prepareToSendIpPacketToTldk(packet, layer2HeaderLength, layer3Length, layer3HeaderLength, layer4HeaderLength);

		// We do this look up once already with discardPacketIf!(packet, self.isNotOneOfOurIpV4Addresses(destinationIpV4Address, destinationEthernetAddress)); above
		if let Some(internet_protocol_addressInformation) = self.ipV4State.ourIpV4Addresses(destinationIpV4Address)
		{
			// Also, need to send when not full after x seconds, otherwise we'll never pass on packets on a slow or low bandwidth connection
			// Can we avoid this look up?
			match layer4Protocol
			{
				Layer4Protocol::Tcp => internet_protocol_addressInformation.tcpReceiveBurstBuffer.bufferAndSendToTldkWhenFull(packet),
				Layer4Protocol::Udp => internet_protocol_addressInformation.udpReceiveBurstBuffer.bufferAndSendToTldkWhenFull(packet),
			}
		}
	}

	#[inline(always)]
	pub fn arpPacket(&mut self, packet: *mut rte_mbuf, layer2HeaderLength: u32, sourceEthernetAddress: *const ether_addr, outboundQueue: &mut TransmitBurstQueue)
	{
		macro_rules! extractAddresses
		{
			($self: ident, $packet: ident, $arpHeader: ident, $arpIpV4Data: ident, $ipState: ident) =>
			{
				{
					const SizeOfIpV4Address: u8 = InternetProtocolVersion4HostAddress::Size as u8;

					discardPacketIf!($packet, HardwareType::is_not_ethernet_2($arpHeader.arp_hrd));
					discardPacketIf!($packet, $arpHeader.arp_pro != ETHER_TYPE_IPv4.to_be());
					discardPacketIf!($packet, $arpHeader.arp_hln != MediaAccessControlAddress::SizeU8);
					discardPacketIf!($packet, $arpHeader.arp_pln != SizeOfIpV4Address);

					let senderHardwareAddress = $arpIpV4Data.arp_sha;
					let targetHardwareAddress = $arpIpV4Data.arp_tha;
					discardPacketIf!($packet, senderHardwareAddress == targetHardwareAddress);
					discardPacketIf!($packet, senderHardwareAddress != unsafe {*sourceEthernetAddress});

					let senderIpV4Address = u32::from_be($arpIpV4Data.arp_sip);
					let targetIpV4Address = u32::from_be($arpIpV4Data.arp_tip);
					discardPacketIf!($packet, senderIpV4Address == targetIpV4Address);
					discardPacketIf!($packet, senderIpV4Address.is_not_valid_unicast());
					discardPacketIf!($packet, targetIpV4Address.is_not_valid_unicast());
					discardPacketIf!($packet, self.isBlackListedSourceIpV4Address(senderIpV4Address));
					// We have an IP address collision; this is pretty much fatal, but we don't panic in case the ARP request is spoofed
					discardPacketIf!($packet, self.isOneOfOurIpV4Addresses(senderIpV4Address));

					(senderHardwareAddress, senderIpV4Address, targetHardwareAddress, targetIpV4Address)
				}
			}
		}

		discardPacketIf!(packet, layer2HeaderLength > EthernetPacketHeader::MaximumSizeU32);
		let correctPacketLength = layer2HeaderLength + size_of::<arp_hdr>() as u32;
		discardPacketIf!(packet, packet.length() < correctPacketLength);

		let enqueuePacket =
			{
				mutatePacketData!(packet, layer2HeaderLength, arp_hdr, arpHeaderPointer,
			{
				let mut arpHeader = unsafe { *arpHeaderPointer };
				let mut arpIpV4Data = arpHeader.arp_data;

				let operationCode = arpHeader.arp_op;

				if likely(operationCode == Operation::REQUEST.to_network_byte_order())
				{
					let (senderHardwareAddress, senderIpV4Address, targetHardwareAddress, targetIpV4Address) = extractAddresses!(self, packet, arpHeader, arpIpV4Data, ipState);

					discardPacketIf!(packet, MediaAccessControlAddress::is_the_destination_ethernet_address_invalid_for_an_address_resolution_protocol_request(destinationEthernetAddress!(packet)));
					discardPacketIf!(packet, MediaAccessControlAddress::isTargetHardwareAddressNotZero(&targetHardwareAddress));

					self.addOrFreshenArpCacheWith(senderIpV4Address, senderHardwareAddress);

					let ourHardwareAddress = discardPacketIfNone!(packet, self.getOurEthernetAddressForIpV4Address(targetIpV4Address));

					mutatePacketData!(packet, 0, ether_hdr, ethernetHeaderPointer,
					{
						let mut ethernetHeader = unsafe { *ethernetHeaderPointer };
						ethernetHeader.d_addr = ethernetHeader.s_addr;
						ethernetHeader.s_addr = ourHardwareAddress;
						(true, ())
					});

					arpHeader.arp_op = Operation::REPLY.to_network_byte_order();
					arpIpV4Data.arp_tip = senderIpV4Address;
					arpIpV4Data.arp_sip = targetIpV4Address;
					arpIpV4Data.arp_tha = senderHardwareAddress;
					arpIpV4Data.arp_sha = ourHardwareAddress;
					(true, true)
				}
				else if likely(operationCode == Operation::REPLY.to_network_byte_order())
				{
					let (senderHardwareAddress, senderIpV4Address, targetHardwareAddress, targetIpV4Address) = extractAddresses!(self, packet, arpHeader, arpIpV4Data, ipState);

					panic!("FINISH ME")
				}
				else
				{
					packet.free();
					return;
				}
			})
			};

		if enqueuePacket
		{
			let result = outboundQueue.pushTransmittingAsRequired(packet);

			// Drop packet if we couldn't transmit it (very unlikely)
			if unlikely(result.is_err())
			{
				result.unwrap_err().free();
			}
		}
	}

	#[inline(always)]
	fn isBlackListedSourceIpV4Address(&self, senderIpV4Address: InternetProtocolVersion4HostAddress) -> bool
	{
		self.sourceIpV4AddressBlackList.isIpAddressBlackListed(&senderIpV4Address)
	}

	#[inline(always)]
	fn isOneOfOurIpV4Addresses(&self, senderIpV4Address: InternetProtocolVersion4HostAddress) -> bool
	{
		self.ourIpV4Addresses.get(&senderIpV4Address).is_some()
	}

	#[inline(always)]
	fn isNotOneOfOurIpV4Addresses(&self, destinationIpV4Address: InternetProtocolVersion4HostAddress, destinationEthernetAddress: *const ether_addr) -> bool
	{
		match self.ourIpV4Addresses.get(&destinationIpV4Address)
		{
			None => false,
			Some(rcRefCellIpAddressInformation) => rcRefCellIpAddressInformation.borrow().ourEthernetAddress == unsafe { *destinationEthernetAddress }
		}
	}

	#[inline(always)]
	fn getOurEthernetAddressForIpV4Address(&self, possiblyOurIpV4Address: InternetProtocolVersion4HostAddress) -> Option<ether_addr>
	{
		self.ourIpV4Addresses.get(&possiblyOurIpV4Address).map(|value| value.ourEthernetAddress.clone())
	}

	#[inline(always)]
	fn addOrFreshenArpCacheWith(&mut self, senderIpV4Address: InternetProtocolVersion4HostAddress, senderHardwareAddress: ether_addr)
	{
		self.arpCache.addOrFreshen(senderIpV4Address, senderHardwareAddress)
	}
}
