// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct EthernetPacketHeader
{
	/// Source and destination addresses.
	pub ethernet_addresses: EthernetAddresses,
	
	/// Ethernet frame size or ether type.
	pub ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize,
}

impl Display for EthernetPacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Into<ether_hdr> for EthernetPacketHeader
{
	#[inline(always)]
	fn into(self) -> ether_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a ether_hdr> for &'a EthernetPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a ether_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a mut ether_hdr> for &'a mut EthernetPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut ether_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<NonNull<ether_hdr>> for &'a mut EthernetPacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<ether_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut EthernetPacketHeader as *mut ether_hdr) }
	}
}

impl<'a> Into<*const ether_hdr> for &'a EthernetPacketHeader
{
	#[inline(always)]
	fn into(self) -> *const ether_hdr
	{
		self as *const EthernetPacketHeader as *const _
	}
}

impl<'a> Into<*mut ether_hdr> for &'a mut EthernetPacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut ether_hdr
	{
		self as *mut EthernetPacketHeader as *mut _
	}
}

impl From<ether_hdr> for EthernetPacketHeader
{
	#[inline(always)]
	fn from(value: ether_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a ether_hdr> for &'a EthernetPacketHeader
{
	#[inline(always)]
	fn from(value: &'a ether_hdr) -> &'a EthernetPacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a mut ether_hdr> for &'a mut EthernetPacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut ether_hdr) -> &'a mut EthernetPacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl EthernetPacketHeader
{
	/// Size of an Ethernet header less EtherType field.
	pub const SizeLessEtherTypeU32: u32 = MediaAccessControlAddress::SizeU32 * 2;
	
	/// Size of an Ethernet header.
	///
	/// Same as `ETHER_HEADER_LENGTH`.
	pub const SizeU32: u32 = Self::SizeLessEtherTypeU32 + EtherType::SizeU32;
	
	/// Size of an Ethernet header.
	///
	/// Same as `ETHER_HEADER_LENGTH`.
	pub const SizeU16: u16 = Self::SizeU32 as u16;
	
	/// Maximum size of an Ethernet header when tagging Virtual LAN data is included.
	pub const MaximumSizeU32: u32 = 127;
	
	pub const SizeU16WithFrameCheckSequence: u16 = Self::SizeU16 + SizeU16OfEthernetCyclicRedundancyCheck;
	
	#[inline(always)]
	pub(crate) fn ethernet_addresses(&self) -> &EthernetAddresses
	{
		&self.ethernet_addresses
	}
	
	#[inline(always)]
	pub(crate) fn potentially_invalid_ether_type(&self) -> EtherType
	{
		self.ether_type_or_legacy_ethernet_frame_size.potentially_invalid_ether_type()
	}
}
