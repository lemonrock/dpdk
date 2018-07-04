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
	pub next_header: u8,
	
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
		// Value 59 (No Next Header) in the Next Header field indicates that there is no next header whatsoever following this one, not even a header of an upper-layer protocol. It means that, from the header's point of view, the IPv6 packet ends right after it: the payload should be empty.[1] There could, however, still be data in the payload if the payload length in the first header of the packet is greater than the length of all extension headers in the packet. This data should be ignored by hosts, but passed unaltered by routers.
		
		// 0 hop-options, 60 destination options [allowed twice], 43 routing, 44 fragment, etc.
		
		// See RFC 8200
		
		//
	}
	
	/// Zero for jumbo frames.
	#[inline(always)]
	pub fn payload_length(&self) -> u16
	{
		// payload length including extension headers.
		//self.payload_length_including_extension_headers.to_native_byte_order_value()
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
}
