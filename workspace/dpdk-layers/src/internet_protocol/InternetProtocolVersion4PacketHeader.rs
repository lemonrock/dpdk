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
	pub next_proto_id: u8,
	
	/// Check sum.
	pub check_sum: NetworkByteOrderEndianU16,
	
	/// Source address.
	pub source_address: InternetProtocolVersion4HostAddress,
	
	/// Destination address.
	pub destination_address: InternetProtocolVersion4HostAddress,
}

impl InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	pub fn is_version_not_4(&self) -> bool
	{
		self.version_and_internet_header_length & 0xF0 != 4 << 4
	}
	
	// If less than 20, then the header length is invalid!
	#[inline(always)]
	pub fn header_length_including_options(&self) -> u8
	{
		self.version_and_internet_header_length & 0x0F << 2
	}
	
	#[inline(always)]
	pub fn hops(&self) -> u8
	{
		self.time_to_live
	}
	
	#[inline(always)]
	pub fn layer_4(&self) -> u8
	{
		self.next_proto_id
	}
	
	#[inline(always)]
	pub fn payload_length(&self) -> u16
	{
		self.header_length_including_options() - self.total_length
	}
	
	/// DifferentiatedServiceCodePoint and ExplicitCongestionNotification.
	#[inline(always)]
	pub fn traffic_class(&self) -> (DifferentiatedServiceCodePoint, ExplicitCongestionNotification)
	{
		let traffic_class = self.type_of_service;
		
		(DifferentiatedServiceCodePoint(traffic_class >> 2), unsafe { transmute(traffic_class & 0b11) })
	}
}
