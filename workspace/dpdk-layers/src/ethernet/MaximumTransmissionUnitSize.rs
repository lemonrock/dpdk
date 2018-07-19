// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Maximum Transmission Unit (MTU) size in bytes.
///
/// Note: With an ethernet MTU of 1500, the TCP Maximum Segment Size (MSS) would be 1460 after subtracting 20 bytes each for the internet protocol version 4 header and TCP header.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct MaximumTransmissionUnitSize(u16);

impl Display for MaximumTransmissionUnitSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl MaximumTransmissionUnitSize
{
	const Rfc791Minimum: u16 = 68;
	
	/// Minimum value for internet protocol (IP) version 4 packets.
	///
	/// Refer to RFC 791.
	pub const MinimumValueForInternetProtocolVersion4: Self = MaximumTransmissionUnitSize(Self::Rfc791Minimum);
	
	/// Minimum value for internet protocol (IP) version 6 packets.
	///
	/// Refer to RFC 2460.
	pub const MinimumValueForInternetProtocolVersion6: Self = MaximumTransmissionUnitSize(1280);
	
	/// When using DS-Lite over PPPoE over Ethernet version 2.
	pub const DsLiteOverPPPoEOverEthernetV2: Self = Self::PPPoEOverEthernetV2.decrease_for_internet_protocol_version_6_header();
	
	/// When using PPPoE over Ethernet version 2.
	pub const PPPoEOverEthernetV2: Self = Self::EthernetV2.decrease_for_pppoe_header();
	
	/// When using Ethernet version 2 with LLC and SNAP.
	pub const EthernetV2WithLlcAndSnap: Self = Self::EthernetV2.decrease_for_llc_and_snap();
	
	/// When using Ethernet version 2.
	pub const EthernetV2WithVirtualLanHeaderDeducted: Self = Self::EthernetV2.decrease_for_virtual_lan_header();

	/// When using Ethernet version 2 for ethernet frame lengths of 1518 bytes.
	///
	/// Most modern networks allow the Virtual LAN header to also not count towards MTU, so use this setting if Virtual LAN headers are present (this implies 1522 byte ethernet frame lengths).
	pub const EthernetV2: Self = Self::from_ethernet_frame_length(EthernetFrameLength::MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames);
	
	/// Maximum value for Jumbo frames.
	pub const PPPoEOverJumboFrames: Self = Self::MaximumJumboValue.decrease_for_pppoe_header();
	
	/// Maximum value for Jumbo frames.
	pub const MaximumJumboValue: Self = Self::from_ethernet_frame_length(EthernetFrameLength::MaximumIncludingCyclicRedundancyCheckWithJumboFrames);
	
	/// From `EthernetFrameLength`.
	#[inline(always)]
	pub const fn from_ethernet_frame_length(ethernet_frame_length: EthernetFrameLength) -> Self
	{
		MaximumTransmissionUnitSize(ethernet_frame_length.to_u16() - EthernetPacketHeader::SizeU16WithFrameCheckSequence)
	}
	
	/// Value of `data_room_size` passed to `rte_pktmbuf_pool_create()` to ensure that segmentation of receive packets is not needed and packet drops do not occur.
	///
	/// Use this value to avoid the need to specify `offloads::DEV_RX_OFFLOAD_SCATTER` for poll-mode drivers (PMDs).
	#[inline(always)]
	pub fn to_data_room_size_for_packet_buffer_pool(self) -> u16
	{
		self.0 + EthernetPacketHeader::SizeU16WithFrameCheckSequence + PacketBuffer::HeadRoom
	}

	/// A safe default is 1500, with 1492 a fallback.
	#[inline(always)]
	pub fn new(bytes: u16) -> Self
	{
		MaximumTransmissionUnitSize(Self::guard(bytes))
	}
	
	/// Decrease for LLC and SNAP header.
	#[inline(always)]
	pub const fn decrease_for_llc_and_snap(self) -> Self
	{
		MaximumTransmissionUnitSize(self.0 - 8)
	}
	
	/// Decrease for PPPoE header.
	#[inline(always)]
	pub const fn decrease_for_pppoe_header(self) -> Self
	{
		#[repr(C, packed)]
		struct PointToPointProtocolOverEthernetHeader
		{
			version_and_type: u8, // 4 bit version 0x1, 4 bit type, 0x1.
			code: u8,
			session_id: NetworkByteOrderEndianU16,
			length: NetworkByteOrderEndianU16,
		}
		
		MaximumTransmissionUnitSize(self.0 - size_of::<PointToPointProtocolOverEthernetHeader>() as u16)
	}
	
	/// Decrease for DS-Lite Internet Protocol Version 6 header.
	#[inline(always)]
	const fn decrease_for_internet_protocol_version_6_header(self) -> Self
	{
		MaximumTransmissionUnitSize(self.0 - (size_of::<ipv6_hdr>() as u16))
	}
	
	/// Decrease for Virtual Lan header.
	///
	/// Note that in some environments the maximum frame length is increased by 4 bytes to accommodate this.
	#[inline(always)]
	pub const fn decrease_for_virtual_lan_header(self) -> Self
	{
		MaximumTransmissionUnitSize(self.0 - VirtualLanPacketHeader::VirtualLanPacketHeaderSizeU16)
	}
	
	/// Decrease for QinQ Virtual Lan followed by Virtual Lan header.
	#[inline(always)]
	pub const fn decrease_for_qinq_virtual_lan_header(self) -> Self
	{
		MaximumTransmissionUnitSize(self.0 - (VirtualLanPacketHeader::QinQVirtualLanPacketHeaderSizeU16 + VirtualLanPacketHeader::VirtualLanPacketHeaderSizeU16))
	}
	
	/// Decrease by provided size.
	#[inline(always)]
	pub fn decrease_by(&self, virtual_lan_or_mac_sec_headers_size: u16) -> Self
	{
		MaximumTransmissionUnitSize(self.0 - virtual_lan_or_mac_sec_headers_size)
	}
	
	/// As u16 value.
	#[inline(always)]
	pub fn as_u16(&self) -> u16
	{
		self.0
	}
	
	/// As usize value.
	#[inline(always)]
	pub fn as_usize(&self) -> usize
	{
		self.0 as usize
	}
	
	/// Conservative frame length for Jumbo frames, ie consider Jumbo frames as being needed if MTU > 1500.
	#[inline(always)]
	pub fn conservative_jumbo_frame_length(&self) -> Option<u16>
	{
		if self.requires_jumbo_frames()
		{
			Some(self.0 + SizeU16OfEthernetCyclicRedundancyCheck)
		}
		else
		{
			None
		}
	}
	
	/// Requires Jumbo frames for this MTU.
	#[inline(always)]
	pub fn requires_jumbo_frames(&self) -> bool
	{
		self > &Self::EthernetV2
	}
	
	/// Packet fragment size for Internet Protocol version 6 packets.
	///
	/// Ensures the IP payload length of all fragments is aligned to a multiple of 8 bytes as per RFC 791 section 2.3.
	#[inline(always)]
	pub fn internet_protocol_version_4_fragment_size(&self) -> u16
	{
		let raw_fragment_size = self.0 - (size_of::<ipv4_hdr>() as u16);
		RTE_ALIGN_FLOOR_u16(raw_fragment_size, IPV4_HDR_FO_ALIGN)
	}
	
	/// Packet fragment size for Internet Protocol version 6 packets.
	#[inline(always)]
	pub fn internet_protocol_version_6_fragment_size(&self) -> u16
	{
		let fragment_size = self.0 - (size_of::<ipv6_hdr>() as u16);
		debug_assert_eq!(fragment_size & !(RTE_IPV6_EHDR_FO_MASK as u16), 0, "Fragment size should be a multiple of 8");
		fragment_size
	}
	
	#[inline(always)]
	fn guard(bytes: u16) -> u16
	{
		// DPDK: "Maximum Jumbo frame length, including CRC".
		const ETHER_MAX_JUMBO_FRAME_LEN: u16 = 0x3F00;
		
		assert!(bytes >= Self::Rfc791Minimum, "The bytes, '{}', must be greater than RFC 791 Minimum ({})", bytes, Self::Rfc791Minimum);
		assert!(bytes <= ETHER_MAX_JUMBO_FRAME_LEN - SizeU16OfEthernetCyclicRedundancyCheck, "The bytes, '{}', must be less than (ETHER_MAX_JUMBO_FRAME_LEN ({}) - SizeU16OfEthernetCyclicRedundancyCheck ({}))", bytes, ETHER_MAX_JUMBO_FRAME_LEN, SizeU16OfEthernetCyclicRedundancyCheck);
		
		bytes
	}
}
