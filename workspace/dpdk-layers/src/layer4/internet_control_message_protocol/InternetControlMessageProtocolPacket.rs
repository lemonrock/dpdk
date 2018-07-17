// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct InternetControlMessageProtocolPacket
{
	/// Header.
	pub header: InternetControlMessageProtocolPacketHeader,
	
	// 4-bytes rest of header.
	pub ident: NetworkByteOrderEndianU16,
	pub sequence_number: NetworkByteOrderEndianU16,
}

impl InternetControlMessageProtocolPacket
{
	/// Processes this packet.
	///
	/// Applies very strict rules to ignore traffic that is not secure.
	#[inline(always)]
	pub fn process_path_mtu_discovery_only<PathMtuDiscovery: Fn()>(&mut self, internet_control_message_protocol_packet_length: usize, internet_header_length: InternetHeaderLength, path_mtu_discovery: PathMtuDiscovery)
	{
		if internet_control_message_protocol_packet_length < size_of::<InternetControlMessageProtocolPacketHeader>()
		{
			return
		}
		
		match (self.header.type_, self.header.code)
		{
			// (Destination Unreachable for Path MTU Discovery, Fragmentation required, and DF flag set)
			(InternetControlMessageProtocolType::DestinationUnreachable, 4) =>
			{
				if internet_control_message_protocol_packet_length != internet_header_length.destination_unreachable_payload_length()
				{
					return
				}
				
				if self.is_internet_checksum_incorrect(internet_control_message_protocol_packet_length)
				{
					return
				}
				
				path_mtu_discovery()
			}
			
			_ => (),
		}
	}
	
	/// After this has executed, the checksum field will be zero.
	#[inline(always)]
	fn is_internet_checksum_incorrect(&mut self, internet_control_message_protocol_packet_length: usize)
	{
		let provided_checksum = self.header.checksum.to_native_byte_order_value();
		let calculated_checksum =
		{
			self.header.checksum.zero();
			let network_byte_order_bytes = unsafe { from_raw_parts(self as *mut Self as *mut u8 as *const u8, internet_control_message_protocol_packet_length)};
			Self::calculate_internet_checksum_native_endian(network_byte_order_bytes, 0)
		};
		provided_checksum != calculated_checksum
	}
	
	/// RFC 1071.
	#[inline(always)]
	fn calculate_internet_checksum_native_endian(network_byte_order_bytes: &[u8], initial_sum: u32) -> u16
	{
		#[inline(always)]
		unsafe fn sum_every_16bits(network_byte_order_bytes: &[u8]) -> u32
		{
			let mut next = network_byte_order_bytes.as_ptr() as *const u16;
			let mut count = network_byte_order_bytes.len();
			
			let mut sum = 0;
			
			while count > 1
			{
				sum += u16::from_be(* next);
				next.offset(1);
				count -= 2;
			}
			
			if count > 0
			{
				sum += * (next as *const u8);
			}
		
			sum
		}
		
		let sum = initial_sum + unsafe { sum_every_16bits(network_byte_order_bytes)};
		
		// Fold 32-bit sum to 16 bit sum.
		let mut folded_to_16_bits = sum;
		while folded_to_16_bits >> 16 != 0
		{
			folded_to_16_bits = (folded_to_16_bits & 0xFFFF) + (folded_to_16_bits >> 16)
		}
		
		!(folded_to_16_bits as u16)
	}
}

/// This is a specialized structure designed to represent a buffer of packet data.
///
/// See RFC 792.
#[repr(C, packed)]
pub union InternetControlMessageProtocolPacketPayload
{
	pub other: PhantomData<u8>,
}

// Destination Unreachable: Internet Header + 64 bits of Original Data Datagram
