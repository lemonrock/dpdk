// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct InternetProtocolVersion4PacketHeader
{
	/// Version and internet header length bit fields.
	pub version_and_internet_header_length: u8,
	
	/// Type of service.
	pub type_of_service: u8,
	
	/// Total length.
	pub total_length: NetworkByteOrderEndianU16,
	
	/// Fragmentation packet identifier.
	pub fragment_identifier: NetworkByteOrderEndianU16,
	
	/// Fragmentation offset.
	pub fragment_offset: NetworkByteOrderEndianU16,
	
	/// Hops.
	pub time_to_live: u8,
	
	/// Layer 4 protocol identifier.
	pub next_proto_id: Layer4ProtocolNumber,
	
	/// Check sum.
	pub check_sum: NetworkByteOrderEndianU16,
	
	/// Source address.
	pub source_address: InternetProtocolVersion4HostAddress,
	
	/// Destination address.
	pub destination_address: InternetProtocolVersion4HostAddress,
}

impl InternetProtocolVersion4PacketHeader
{
	pub(crate) const HeaderSize: usize = size_of::<Self>();
	
	pub(crate) const HeaderSizeU8: u8 = Self::HeaderSize as u8;
	
	pub(crate) const HeaderSizeU16: u16 = Self::HeaderSize as u16;
	
	const ReservedFragmentFlag: u16 = 0b1000_0000_0000_0000;
	
	const DoNotFragmentFlag: u16 = 0b0100_0000_0000_0000;
	
	const MoreFragmentsFlag: u16 = 0b0010_0000_0000_0000;
	
	#[inline(always)]
	pub fn is_version_not_4(&self) -> bool
	{
		self.version_and_internet_header_length & 0xF0 != 4 << 4
	}
	
	/// RFC 6864 Section 4: "Atomic datagrams: (DF==1)&&(MF==0)&&(frag_offset==0)
	/// Non-atomic datagrams: (DF==0)||(MF==1)||(frag_offset>0)".
	#[inline(always)]
	pub fn has_invalid_fragmentation_flags_or_identification(&self) -> bool
	{
		const FragmentFlagMask: u16 = InternetProtocolVersion4PacketHeader::ReservedFragmentFlag | InternetProtocolVersion4PacketHeader::DoNotFragmentFlag | InternetProtocolVersion4PacketHeader::MoreFragmentsFlag;
		
		const UnfragmentedOrLastFragment: u16 = 0;
		
		let r = self.fragment_offset.to_network_byte_order_value() & FragmentFlagMask.to_be();
		
		if r == Self::DoNotFragmentFlag.to_be()
		{
			// Strictly speaking, checking the fragment identifier is non-zero VIOLATES RFC 6864 section 4.1 paragraph 5.
			self.fragment_offset_is_not_zero() || if cfg!(feature = "drop-ipv4-packets-with-do-not-fragment-and-non-zero-identification")
			{
				self.fragment_identifier.is_not_zero()
			}
			else
			{
				false
			}
		}
		else if r == UnfragmentedOrLastFragment || r == Self::MoreFragmentsFlag.to_be()
		{
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	fn fragment_offset_is_not_zero(&self) -> bool
	{
		self.fragment_offset.to_network_byte_order_value() != 0
	}
	
	#[inline(always)]
	pub fn header_length_including_options(&self) -> u8
	{
		self.version_and_internet_header_length & 0x0F << 2
	}
	
	#[inline(always)]
	pub fn total_length(&self) -> u16
	{
		self.total_length.to_network_byte_order_value()
	}
	
	#[inline(always)]
	pub fn hops(&self) -> u8
	{
		self.time_to_live
	}
	
	#[inline(always)]
	pub fn layer_4(&self) -> Layer4ProtocolNumber
	{
		self.next_proto_id
	}
	
	#[inline(always)]
	pub fn payload_length(&self) -> u16
	{
		self.total_length.to_native_byte_order_value() - (self.header_length_including_options() as u16)
	}
	
	/// DifferentiatedServiceCodePoint and ExplicitCongestionNotification.
	#[inline(always)]
	pub fn traffic_class(&self) -> (DifferentiatedServiceCodePoint, ExplicitCongestionNotification)
	{
		let traffic_class = self.type_of_service;
		
		(DifferentiatedServiceCodePoint(traffic_class >> 2), unsafe { transmute(traffic_class & 0b11) })
	}
	
	/// Checks if an internet protocol (IP) version 4 packet is fragmented.
	#[inline(always)]
	pub fn is_fragmented(&self) -> bool
	{
		const OffsetMask: u16 = InternetProtocolVersion4PacketHeader::MoreFragmentsFlag - 1;
		
		self.fragment_offset.to_network_byte_order_value() & (Self::MoreFragmentsFlag | OffsetMask).to_be() != 0
	}
	
	#[inline(always)]
	pub fn to_dpdk(&self) -> NonNull<ipv4_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *const Self as *mut Self as *mut ipv4_hdr) }
	}
}
