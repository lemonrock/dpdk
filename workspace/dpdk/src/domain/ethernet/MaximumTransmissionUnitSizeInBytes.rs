// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Maximum Transmission Unit (MTU) size in bytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct MaximumTransmissionUnitSizeInBytes(u16);

impl MaximumTransmissionUnitSizeInBytes
{
	/// Minimum value.
	pub const Minimum: Self = MaximumTransmissionUnitSizeInBytes(ETHER_MIN_MTU);
	
	/// When using DS-Lite over PPPoE over Ethernet version 2.
	pub const DsLiteOverPPPoEOverEthernetV2: Self = MaximumTransmissionUnitSizeInBytes(1452);
	
	/// When using PPPoE over Ethernet version 2.
	pub const PPPoEOverEthernetV2: Self = MaximumTransmissionUnitSizeInBytes(1492);
	
	/// When using Ethernet version 2 with LLC and SNAP.
	pub const EthernetV2WithLlcAndSnap: Self = MaximumTransmissionUnitSizeInBytes(1492);
	
	/// When using Ethernet version 2 without LLC or SNAP.
	pub const EthernetV2: Self = MaximumTransmissionUnitSizeInBytes(ETHER_MTU);
	
	/// TLDK commonly used maximum.
	pub const TldkValue: Self = MaximumTransmissionUnitSizeInBytes(1514);
	
	/// Maximum value for Jumbo frames.
	pub const MaximumJumboValue: Self = MaximumTransmissionUnitSizeInBytes(ETHER_MAX_JUMBO_FRAME_LEN - ETHER_CRC_LEN);

	/// A safe default is 1500, with 1492 a fallback.
	#[inline(always)]
	pub fn new(maximum_transmission_unit_in_bytes: u16) -> Self
	{
		MaximumTransmissionUnitSizeInBytes(Self::guard(maximum_transmission_unit_in_bytes))
	}
	
	/// If using PPPoE, reduces the specified size by the size of the PPPoE header.
	#[inline(always)]
	pub fn new_if_using_pppoe(maximum_transmission_unit_in_bytes_will_be_made_smaller: u16) -> Self
	{
		MaximumTransmissionUnitSizeInBytes(Self::guard(maximum_transmission_unit_in_bytes_will_be_made_smaller - SizeOfPPPoEHeader))
	}
	
	/// Decrease by provided size.
	#[inline(always)]
	pub fn decrease_by(&self, virtual_lan_headers_size: u16) -> Self
	{
		MaximumTransmissionUnitSizeInBytes(self.0 - virtual_lan_headers_size)
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
			Some(self.0 + ETHER_CRC_LEN)
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
		self > Self::EthernetV2
	}
	
	/// Packet fragment size for Internet Protocol version 6 packets.
	///
	/// Ensures the IP payload length of all fragments is aligned to a multiple of 8 bytes as per RFC791 section 2.3.
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
	fn guard(maximum_transmission_unit_in_bytes: u16) -> u16
	{
		assert!(maximum_transmission_unit_in_bytes >= ETHER_MIN_MTU, "The maximum_transmission_unit_in_bytes, '{}', must be greater than ETHER_MIN_MTU ({})", maximum_transmission_unit_in_bytes, ETHER_MIN_MTU);
		assert!(maximum_transmission_unit_in_bytes <= ETHER_MAX_JUMBO_FRAME_LEN - ETHER_CRC_LEN, "The maximum_transmission_unit_in_bytes, '{}', must be less than (ETHER_MAX_JUMBO_FRAME_LEN ({}) - ETHER_CRC_LEN ({}))", maximum_transmission_unit_in_bytes, ETHER_MAX_JUMBO_FRAME_LEN, ETHER_CRC_LEN);
		
		maximum_transmission_unit_in_bytes
	}
}
