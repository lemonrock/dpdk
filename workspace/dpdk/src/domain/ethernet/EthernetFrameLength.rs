// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents an Ethernet Frame Length which includes the ethernet header length and cyclic redundancy check length.
///
/// Defaults to `MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames`.
///
/// The cyclic redundancy check is also sometimes called the trailing frame check sequence, or trailing FCS.
///
/// Note that the ethernet frame length does not include the 8-byte ethernet preamble or the inter-packet gap (IPG 12 bytes on 100Mbit, 1 byte on 100Gbit) as these are Layer 1 overheads and an ethernet frame is logically at layer 2.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EthernetFrameLength(u16);

impl Default for EthernetFrameLength
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames
	}
}

impl EthernetFrameLength
{
	/// Minimum frame length including cyclic redundancy check (CRC).
	pub const MinimumIncludingCyclicRedundancyCheck: Self = EthernetFrameLength(64);
	
	/// Maximum frame length including cyclic redundancy check (CRC) if jumbo frames are not possible.
	pub const MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames: Self = EthernetFrameLength(1518);
	
	/// Maximum frame length including cyclic redundancy check (CRC) if jumbo frames are not possible but 802.1 Virtual LAN tags are allowed in addition.
	pub const MaximumIncludingCyclicRedundancyCheckWithoutJumboFramesButVirtualLanAdditionAllowed: Self = Self::MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames.0 + (SizeOfVlanHeader as u16);
	
	pub const MultiTaggedEnvelopFramesCyclicRedundancyCheck: Self = EthernetFrameLength(2000);
	
	pub const TypicalModernEquipmentJumboFrameLengthMaximumIncludingCyclicRedundancyCheck: Self = EthernetFrameLength(9216);
	
	/// These would be considered by some to be 'super-jumbo' frames (SJF).
	pub const MaximumIncludingCyclicRedundancyCheckWithJumboFrames: Self = EthernetFrameLength(16_128);
	
	/// Panics with an assertion failure if out-of-range.
	#[inline(always)]
	pub fn try_from_without_jumbo_frames(ethernet_frame_length: u16) -> Self
	{
		assert!(ethernet_frame_length >= Self::MinimumIncludingCyclicRedundancyCheck.0, "ethernet_frame_length '{}' is less than minimum '{}'", ethernet_frame_length, Self::MinimumIncludingCyclicRedundancyCheck.0);
		assert!(ethernet_frame_length <= Self::MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames.0, "ethernet_frame_length '{}' is less than maximum '{}'", ethernet_frame_length, Self::MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames.0);
		EthernetFrameLength(ethernet_frame_length)
	}
	
	/// Panics with an assertion failure if out-of-range.
	#[inline(always)]
	pub fn try_from_without_jumbo_frames_but_virtual_lan_addition_allowed(ethernet_frame_length: u16) -> Self
	{
		assert!(ethernet_frame_length >= Self::MinimumIncludingCyclicRedundancyCheck.0, "ethernet_frame_length '{}' is less than minimum '{}'", ethernet_frame_length, Self::MinimumIncludingCyclicRedundancyCheck.0);
		assert!(ethernet_frame_length <= Self::MaximumIncludingCyclicRedundancyCheckWithoutJumboFramesButVirtualLanAdditionAllowed.0, "ethernet_frame_length '{}' is less than maximum '{}'", ethernet_frame_length, Self::MaximumIncludingCyclicRedundancyCheckWithoutJumboFramesButVirtualLanAdditionAllowed.0);
		EthernetFrameLength(ethernet_frame_length)
	}
	
	/// Panics with an assertion failure if out-of-range.
	#[inline(always)]
	pub fn try_from_with_jumbo_frames(ethernet_frame_length: u16) -> Self
	{
		assert!(ethernet_frame_length >= Self::MinimumIncludingCyclicRedundancyCheck.0, "ethernet_frame_length '{}' is less than minimum '{}'", ethernet_frame_length, Self::MinimumIncludingCyclicRedundancyCheck.0);
		assert!(ethernet_frame_length <= Self::MaximumIncludingCyclicRedundancyCheckWithJumboFrames.0, "ethernet_frame_length '{}' is less than maximum '{}'", ethernet_frame_length, Self::MaximumIncludingCyclicRedundancyCheckWithJumboFrames.0);
		EthernetFrameLength(ethernet_frame_length)
	}
	
	/// Calculates maximum transmission unit size in bytes.
	#[inline(always)]
	pub const fn calculate_maximum_transmission_unit_size_in_bytes(self) -> MaximumTransmissionUnitSize
	{
		MaximumTransmissionUnitSize::from_ethernet_frame_length(self)
	}
	
	/// Converts to an `u16`.
	#[inline(always)]
	pub fn to_u16(self) -> u16
	{
		self.0
	}
	
	/// Does this frame size imply jumbo frames?
	#[inline(always)]
	pub fn implies_jumbo_frames(self) -> bool
	{
		self > MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames.0
	}
}
