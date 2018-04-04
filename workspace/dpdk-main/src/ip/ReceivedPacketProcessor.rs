// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



// See: http://www.dpdk.org/ml/archives/users/2017-February/001484.html . All non-RSS Layer 4 frames are received by Queue 0, which means we have one writer, many readers for ArpCache

// See rte_mbuf_prefetch_part1() - excludes length fields
// See rte_mbuf_prefetch_part2(struct rte_mbuf *m) - length fields, segment fields (used by rte_pktmbuf_read)

// TODO: Decide if and how we free...
// NOTE: rte_eth_tx_burst() frees all packets it managed to send; we only have to free those we don't re-use


// TODO: Replace if() checks in places (eg packetDataUser) with macros

// TODO: IPv4 / v6 fragmentation (best done post ptype processing AND post deciding which level 4 protocols we support)
// TODO: IPv6 unicast check is broken
// TODO: IPv6 options...
// TODO: IPv4 TOS / https://en.wikipedia.org/wiki/Type_of_service and other options... //www.iana.org/assignments/ip-parameters/ip-parameters.xhtml
// TODO: Arp reply to request - need to send out
// TODO: Do something for REPLY
// TODO: Layer4 checksum check (PKT_TX_*) for TCP / UDP / ICMPv4 / ICMPv6
// Frankly, it's not obvious that we need ICMPv4 at all. We do need ICMPv6.

// TODO: Discard tunneled packets (ie protocol is not TCP or UDP)

// TODO: Defragment the packet



#[allow(missing_debug_implementations)]
pub struct ReceivedPacketProcessor
{
	pub destinations: Destinations,
	pub outboundQueue: TransmitBurstQueue,
}

impl PacketProcessor for ReceivedPacketProcessor
{
	#[inline(always)]
	fn processPacket(&mut self, packet: *mut rte_mbuf)
	{
		discardPacketIf!(packet, packet.length() < MinimumSizeOfEthernetPacketSizeAssumingArp);
		
		let sourceEthernetAddress = sourceEthernetAddress!(packet);
		// We have a MAC collision; this is pretty much fatal, but we don't panic in case the originator is spoofed (eg an ARP request spoof)
		discardPacketIf!(packet, self.destinations.isSourceEthernetAddressOneOfOurEthernetAddresses(sourceEthernetAddress));
		discardPacketIf!(packet, self.destinations.isSourceEthernetAddressInvalidOrBlackListed(sourceEthernetAddress));
		
		match etherType!(packet, SizeOfEthernetHeaderLessEtherType)
		{
			ETHER_TYPE_IPv6 =>
			{
				let ipState = discardPacketIfNone!(packet, self.destinations.ipState(NoVirtualLanKey));
				ipState.ipV6Packet(packet, SizeOfEthernetHeader)
			},
			
			ETHER_TYPE_IPv4 =>
			{
				let ipState = discardPacketIfNone!(packet, self.destinations.ipState(NoVirtualLanKey));
				ipState.ipV4Packet(packet, SizeOfEthernetHeader)
			},
			
			ETHER_TYPE_ARP =>
			{
				let ipState = discardPacketIfNone!(packet, self.destinations.ipState(NoVirtualLanKey));
				ipState.arpPacket(packet, SizeOfEthernetHeader, sourceEthernetAddress, &mut self.outboundQueue)
			},
			
			ETHER_TYPE_VLAN =>
			{
				discardPacketIf!(packet, packet.length() < MinimumSizeOfVlanPacketAssumingArp);
				self.vlanPacket(packet, SizeOfEthernetHeader, None, sourceEthernetAddress)
			},
			
			ETHER_TYPE_QINQ =>
			{
				discardPacketIf!(packet, packet.length() < MinimumSizeOfQinQPacketAssumingArp);
				self.qInQPacket(packet, sourceEthernetAddress)
			},
			
			_ => packet.free()
		}
	}
}

impl ReceivedPacketProcessor
{
	#[inline(always)]
	fn vlanPacket(&mut self, packet: *mut rte_mbuf, offsetToJustAfterOurEtherTypeOfVlan: u32, vlanQinQId: Option<VirtualLanId>, sourceEthernetAddress: *const ether_addr)
	{
		#[macro_export]
		macro_rules! common
		{
			($packet: ident, $offsetToJustAfterOurEtherTypeOfVlan: ident, $ourOffset: ident, $vlanQinQId: ident) =>
			{
				{
					let tagControlInformation = if likely($packet.wasVlanTagControlInformationStripped())
					{
						$packet.strippedVlanTagControlInformation()
					}
					else
					{
						tagControlValue!($packet, $offsetToJustAfterOurEtherTypeOfVlan)
					};
					
					let virtualLanId = discardPacketIfDropEligibleBitSetOrTagControlInformationIsInvalid!($packet, tagControlInformation);
					let nextOffset = $ourOffset + SizeOfEtherType;
					(nextOffset, ($vlanQinQId, virtualLanId))
				}
			}
		}
		
		let ourOffset = offsetToJustAfterOurEtherTypeOfVlan + SizeOfTagControlInformation;
		
		match etherType!(packet, ourOffset)
		{
			ETHER_TYPE_IPv6 =>
			{
				let (nextOffset, virtualLanKey) = common!(packet, offsetToJustAfterOurEtherTypeOfVlan, ourOffset, vlanQinQId);
				let ipState = discardPacketIfNone!(packet, self.destinations.ipState(virtualLanKey));
				ipState.ipV6Packet(packet, nextOffset)
			},
			
			ETHER_TYPE_IPv4 =>
			{
				let (nextOffset, virtualLanKey) = common!(packet, offsetToJustAfterOurEtherTypeOfVlan, ourOffset, vlanQinQId);
				let ipState = discardPacketIfNone!(packet, self.destinations.ipState(virtualLanKey));
				ipState.ipV4Packet(packet, nextOffset)
			},
			
			ETHER_TYPE_ARP =>
			{
				let (nextOffset, virtualLanKey) = common!(packet, offsetToJustAfterOurEtherTypeOfVlan, ourOffset, vlanQinQId);
				let ipState = discardPacketIfNone!(packet, self.destinations.ipState(virtualLanKey));
				ipState.arpPacket(packet, nextOffset, sourceEthernetAddress, &mut self.outboundQueue)
			},
			
			_ => packet.free()
		}
	}
	
	fn qInQPacket(&mut self, packet: *mut rte_mbuf, sourceEthernetAddress: *const ether_addr)
	{
		// We don't handle triple-tagged packets ETHER_TYPE_QINQ (or more); whilst valid, they are exceedingly rare; this is the only other possibility at this offset apart from an IEEE 802.3 frame
		let etherType = etherType!(packet, OffsetToVlanHeaderIfVlanQinQ);
		discardPacketIf!(packet, etherType != ETHER_TYPE_VLAN);
		
		let tagControlInformation = if likely(packet.wasVlanQinQTagControlInformationStripped())
		{
			packet.strippedVlanQInQTagControlInformation()
		}
		else
		{
			tagControlValue!(packet, OffsetToVlanQinQTciValue)
		};
		
		let virtualLanId = discardPacketIfDropEligibleBitSetOrTagControlInformationIsInvalid!(packet, tagControlInformation);
		self.vlanPacket(packet, OffsetToJustAfterVlanHeaderEtherTypeIfVlanQinQ, virtualLanId, sourceEthernetAddress)
	}
}
