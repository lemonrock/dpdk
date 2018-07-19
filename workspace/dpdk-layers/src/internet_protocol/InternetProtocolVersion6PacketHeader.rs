// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternetProtocolVersion6PacketHeader
{
	/// Version, traffic class and flow label bit fields.
	pub version_and_traffic_class_and_flow_label: [u8; 4],
	
	/// Payload length
	pub payload_length_including_extension_headers: NetworkByteOrderEndianU16,
	
	/// Next header.
	pub next_header: ExtensionHeaderTypeOrLayer4ProtocolNumber,
	
	/// Hop limits.
	pub hop_limits: u8,
	
	/// Source address.
	pub source_address: InternetProtocolVersion6HostAddress,
	
	/// Destination address.
	pub destination_address: InternetProtocolVersion6HostAddress,
	
	/// Extension header or payload pointer.
	pub extension_header_or_payload: PhantomData<u8>,
}

impl Display for InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Into<ipv6_hdr> for InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> ipv6_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a ipv6_hdr> for &'a InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a ipv6_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a mut ipv6_hdr> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut ipv6_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<NonNull<ipv6_hdr>> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<ipv6_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut InternetProtocolVersion6PacketHeader as *mut ipv6_hdr) }
	}
}

impl<'a> Into<*const ipv6_hdr> for &'a InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> *const ipv6_hdr
	{
		self as *const InternetProtocolVersion6PacketHeader as *const _
	}
}

impl<'a> Into<*mut ipv6_hdr> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut ipv6_hdr
	{
		self as *mut InternetProtocolVersion6PacketHeader as *mut _
	}
}

impl From<ipv6_hdr> for InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn from(value: ipv6_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a ipv6_hdr> for &'a InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn from(value: &'a ipv6_hdr) -> &'a InternetProtocolVersion6PacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a mut ipv6_hdr> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut ipv6_hdr) -> &'a mut InternetProtocolVersion6PacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl InternetProtocolVersion6PacketHeader
{
	pub(crate) const HeaderSizeU16: u16 = size_of::<Self>() as u16;
	
	#[inline(always)]
	pub(crate) fn is_version_not_6(&self) -> bool
	{
		(unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(0) }) >> 4 != 6
	}
	
	#[inline(always)]
	pub fn hops(&self) -> u8
	{
		self.hop_limits
	}
	
	/// DifferentiatedServiceCodePoint and ExplicitCongestionNotification.
	#[inline(always)]
	pub fn traffic_class(&self) -> (DifferentiatedServiceCodePoint, ExplicitCongestionNotification)
	{
		const TrafficClassBits: u32 = 20;
		const TrafficClassMask: u32 = 0b1111_1111 << TrafficClassBits;
		
		let traffic_class = ((u32::from_be(u32::from_bytes(self.version_and_traffic_class_and_flow_label)) & TrafficClassMask) >> TrafficClassBits) as u8;
		
		(DifferentiatedServiceCodePoint(traffic_class >> 2), unsafe { transmute(traffic_class & 0b11) })
	}
	
	/// 20-bit flow-label
	#[inline(always)]
	pub fn flow_label(&self) -> u32
	{
		const FlowLabelMask: u32 = 0xFFF;
		
		u32::from_be(u32::from_bytes(self.version_and_traffic_class_and_flow_label)) & FlowLabelMask
	}
	
	/// If an internet protocol (IP) version 6 packet is fragmented, gets the fragmentation header.
	///
	/// Current implementation is naive and only checks first extension header.
	///
	/// Returns null if no header is present.
	#[inline(always)]
	pub fn is_fragmented(&self) -> *const ipv6_extension_fragment
	{
		if self.next_header.extension_header_type == ExtensionHeaderType::Fragment
		{
			unsafe { (self as *const Self as *const ipv6_hdr).offset(1) as *const ipv6_extension_fragment }
		}
		else
		{
			null()
		}
	}
}
