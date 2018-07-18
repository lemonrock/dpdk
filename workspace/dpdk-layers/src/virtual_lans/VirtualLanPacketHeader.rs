// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An IEEE 802.1Q or 802.1ad virtual lan tagged packet header.
///
/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct VirtualLanPacketHeader
{
	/// Tag Control Information.
	pub tag_control_information: VirtualLanPacketTagControlInformation,
	
	/// Ethernet frame size or ether type.
	pub ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize,
}

impl VirtualLanPacketHeader
{
	/// An IEEE802.1ad virtual LAN header (excluding IEEE802.1Q virtual lan header) size.
	pub const QinQVirtualLanPacketHeaderSize: usize = 4;
	
	/// An IEEE802.1ad virtual LAN header (excluding IEEE802.1Q virtual lan header) size.
	pub const QinQVirtualLanPacketHeaderSizeU16: u16 = Self::QinQVirtualLanPacketHeaderSize as u16;
	
	/// An IEEE802.1Q virtual LAN header size.
	pub const VirtualLanPacketHeaderSize: usize = 4;
	
	/// An IEEE802.1Q virtual LAN header size.
	pub const VirtualLanPacketHeaderSizeU16: u16 = Self::VirtualLanPacketHeaderSize as u16;
	
	#[inline(always)]
	pub(crate) fn potentially_invalid_ether_type(&self) -> EtherType
	{
		unsafe { self.ether_type_or_legacy_ethernet_frame_size.ether_type }
	}
}
