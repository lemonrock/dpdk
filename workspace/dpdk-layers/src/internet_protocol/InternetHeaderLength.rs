// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Header Length (IHL).
///
/// A 4-bit field in bits 4:7 of the first field of the internet protocol (IP) version 4 packet header. The bits represent the number of 32-bit words in the header.
///
/// Minimum value is 5 (20 bytes).
///
/// Maximum value if 15 (60 bytes).
///
/// Stored as number of bytes, 5 - 15 inclusive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternetHeaderLength(u8);

impl InternetHeaderLength
{
	/// Length in bytes.
	#[inline(always)]
	pub fn from_original_field_in_packet(original_value: u8) -> Result<Self, ()>
	{
		const BitsPerWord: u8 = 32;
		const BitsPerOctet: u8 = 8;
		const BytesScalar: u8 = BitsPerWord / BitsPerOctet;
		
		let masked_original_value = original_value & 0xF0 >> 4;
		if masked_original_value < 5
		{
			Err(())
		}
		else
		{
			Ok(InternetHeaderLength((masked_original_value) * BytesScalar))
		}
	}
	
	/// Length in bytes.
	#[inline(always)]
	pub fn to_u8(self) -> u8
	{
		self.0
	}
	
	/// Packet length for an internet control message protocol (ICMP) destination unreachable packet.
	#[inline(always)]
	pub fn internet_control_message_protocol_destination_unreachable_packet_length(self) -> usize
	{
		const _64_Bits: u8 = 8;
		let payload_length = self.to_u8() + _64_Bits;
		size_of::<InternetControlMessageProtocolPacketHeader>() + (payload_length as usize)
	}
}
