// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


macro_rules! discardPacket
{
	($packet: ident) =>
	{
		{
			$packet.free();
			return;
		}
	}
}

macro_rules! fixUpTcpForTldk
{
	($packet: ident, $layer2HeaderLength: ident, $layer3HeaderLength: ident, $ipv4OrIpV6: ident) =>
	{
		{
			// Fix up packet types for TLDK
			(unsafe {*$packet}).packet_type |= ($ipv4OrIpV6 | RTE_PTYPE_L4_TCP);
			
			let tcpHeaderOffset = $layer2HeaderLength + $layer3HeaderLength;
			let layer4HeaderLength = packetData!($packet, tcpHeaderOffset, tcp_hdr, tcpHeader
			{
				const Top4BitsMask: u8 = 0xF0;
				((unsafe { *tcpHeader }).data_off & Top4BitsMask) >> 2;
			});
			discardPacketIf!(packet, layer4HeaderLength < 20);
			
			(Layer4Protocol::Tcp, layer4HeaderLength)
		}
	}
}

macro_rules! fixUpUdpForTldk
{
	($packet: ident, $layer2HeaderLength: ident, $layer3HeaderLength: ident, $ipv4OrIpV6: ident) =>
	{
		{
			// Fix up packet types for TLDK
			(unsafe {*$packet}).packet_type |= ($ipv4OrIpV6 | RTE_PTYPE_L4_UDP)
			
			(Layer4Protocol::Udp, size_of::<udp_hdr>() as u8)
		}
	}
}

pub struct IpState
{
	pub ipV4State: IpV4State,
	pub ipV6State: IpV6State,
}

impl IpState
{
	pub const DpdkMaximumLayer2HeaderLength: u32 = 255;
	
	pub const DpdkMaximumLayer3HeaderLength: u32 = 255;
	
	pub const DpdkMaximumLayer4HeaderLength: u32 = 255;
	
	#[inline(always)]
	pub fn ipV6Packet(&mut self, packet: *mut rte_mbuf, layer2HeaderLength: u32)
	{
		self.ipV6State.ipV6Packet(packet, layer2HeaderLength)
	}
	
	#[inline(always)]
	pub fn ipV4Packet(&mut self, packet: *mut rte_mbuf, layer2HeaderLength: u32)
	{
		self.ipV4State.ipV4Packet(packet, layer2HeaderLength)
	}
	
	#[inline(always)]
	pub fn arpPacket(&mut self, packet: *mut rte_mbuf, layer2HeaderLength: u32, sourceEthernetAddress: *const ether_addr, outboundQueue: &mut TransmitBurstQueue)
	{
		self.ipV4State.arpPacket(packet, packet, layer2HeaderLength, sourceEthernetAddress, outboundQueue)
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn prepareToSendIpPacketToTldk(packet: *mut rte_mbuf, layer2HeaderLength: u32, layer3Length: u32, layer3HeaderLength: u32, layer4HeaderLength: u8)
	{
		debug_assert!(layer3Length >= layer3HeaderLength, "layer3Length is too small");
		debug_assert!(layer2HeaderLength < Self::DpdkMaximumLayer2HeaderLength, "layer2HeaderLength is too large");
		debug_assert!(layer3HeaderLength < Self::DpdkMaximumLayer3HeaderLength, "layer3HeaderLength is too large");
		
		// fix up lengths
		unsafe { rust_rte_pktmbuf_setMajorLengthBitfields(packet, layer2HeaderLength as u8, layer3HeaderLength as u8, layer4HeaderLength) };
		
		// Fix up packet lengths
		let correctPacketLength = layer2HeaderLength + layer3Length;
		let packetLength = packet.length();
		discardPacketIf!(packet, correctPacketLength > packetLength);
		
		if correctPacketLength < packetLength
		{
			let trimOff = packetLength - correctPacketLength;
			unsafe { rust_rte_pktmbuf_trim(packet, trimOff as u16) };
		}
	}
}
