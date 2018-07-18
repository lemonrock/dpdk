// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
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

impl InternetProtocolVersion6PacketHeader
{
	pub(crate) const HeaderSizeU16: u16 = size_of::<Self>() as u16;
	
	#[inline(always)]
	pub fn is_version_not_6(&self) -> bool
	{
		(unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(0) }) >> 4 != 6
	}
	
	#[inline(always)]
	pub fn hops(&self) -> u8
	{
		self.hop_limits
	}
	
	/// Layer 4 (after parsing self.next_header).
	#[inline(always)]
	pub fn layer_4(&self) -> u8
	{
		xxx;
	}
	
	/// Zero for jumbo frames.
	#[inline(always)]
	pub fn payload_length(&self) -> u16
	{
		xxx;
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
	
	#[inline(always)]
	pub fn to_dpdk(&self) -> NonNull<ipv6_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *const Self as *mut Self as *mut ipv6_hdr) }
	}
}
