// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An IEEE 802.1Q or 802.1ad virtual lan tagged packet header.
///
/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct VirtualLanPacketHeader
{
	/// Tag Control Information.
	pub tag_control_information: TagControlInformation,
	
	/// Ethernet frame size or ether type.
	pub ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize,
}

impl Display for VirtualLanPacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Into<vlan_hdr> for VirtualLanPacketHeader
{
	#[inline(always)]
	fn into(self) -> vlan_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a vlan_hdr> for &'a VirtualLanPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a vlan_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a mut vlan_hdr> for &'a mut VirtualLanPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut vlan_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<NonNull<vlan_hdr>> for &'a mut VirtualLanPacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<vlan_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut VirtualLanPacketHeader as *mut vlan_hdr) }
	}
}

impl<'a> Into<*const vlan_hdr> for &'a VirtualLanPacketHeader
{
	#[inline(always)]
	fn into(self) -> *const vlan_hdr
	{
		self as *const VirtualLanPacketHeader as *const _
	}
}

impl<'a> Into<*mut vlan_hdr> for &'a mut VirtualLanPacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut vlan_hdr
	{
		self as *mut VirtualLanPacketHeader as *mut _
	}
}

impl From<vlan_hdr> for VirtualLanPacketHeader
{
	#[inline(always)]
	fn from(value: vlan_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a vlan_hdr> for &'a VirtualLanPacketHeader
{
	#[inline(always)]
	fn from(value: &'a vlan_hdr) -> &'a VirtualLanPacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a mut vlan_hdr> for &'a mut VirtualLanPacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut vlan_hdr) -> &'a mut VirtualLanPacketHeader
	{
		unsafe { transmute(value) }
	}
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
	pub(crate) fn tag_control_information(&self) -> TagControlInformation
	{
		self.tag_control_information
	}
	
	#[inline(always)]
	pub(crate) fn potentially_invalid_ether_type(&self) -> EtherType
	{
		self.ether_type_or_legacy_ethernet_frame_size.potentially_invalid_ether_type()
	}
}
