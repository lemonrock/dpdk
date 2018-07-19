// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This type wraps an u16 such that it is in network byte order (big endian) form.
///
/// Internally, stores an u16 as a big endian bit pattern.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NetworkByteOrderEndianU16(u16);

impl Display for NetworkByteOrderEndianU16
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.to_native_byte_order_value())
	}
}

impl Debug for NetworkByteOrderEndianU16
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.to_native_byte_order_value())
	}
}

impl Default for NetworkByteOrderEndianU16
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Zero
	}
}

impl NetworkByteOrderEndianU16
{
	/// Zero.
	pub const Zero: Self = NetworkByteOrderEndianU16(0);
	
	/// Create from a network byte order value.
	///
	/// Assumes the value is already stored as a big endian bit pattern.
	#[inline(always)]
	pub const fn from_network_byte_order_value(big_endian_value: u16) -> Self
	{
		NetworkByteOrderEndianU16(big_endian_value)
	}
	
	/// Create from a native byte order value.
	///
	/// Assumes the value is already stored as a native endian bit pattern, and so will swap bytes on Little Endian platforms.
	#[inline(always)]
	pub fn from_native_byte_order_value(native_endian_value: u16) -> Self
	{
		NetworkByteOrderEndianU16(native_endian_value.to_be())
	}
	
	/// To network byte order.
	#[inline(always)]
	pub fn to_network_byte_order_value(self) -> u16
	{
		self.0
	}
	
	/// To native byte order.
	///
	/// On a Little Endian platform, will swap bytes.
	#[inline(always)]
	pub fn to_native_byte_order_value(self) -> u16
	{
		u16::from_be(self.0)
	}
	
	/// Zeros out data.
	#[inline(always)]
	pub fn zero(&mut self)
	{
		self.0 = 0;
	}
	
	#[inline(always)]
	pub fn is_not_zero(self) -> bool
	{
		self.0 != 0
	}
	
	/// High order bits
	///
	/// Returns the top 8-bits.
	#[inline(always)]
	pub fn high_order_bits(self) -> u8
	{
		if cfg!(target_endian = "big")
		{
			(self.0 >> 8) as u8
		}
		else
		{
			(self.0 | 0x00FF) as u8
		}
	}
	
	/// Low order bits
	///
	/// Returns the bottom 8-bits.
	#[inline(always)]
	pub fn low_order_bits(self) -> u8
	{
		if cfg!(target_endian = "big")
		{
			(self.0 | 0x00FF) as u8
		}
		else
		{
			(self.0 >> 8) as u8
		}
	}
	
	/// To native byte order.
	///
	/// On a Little Endian platform, will swap bytes.
	#[inline(always)]
	pub fn mask_lower_12_bits(self) -> Self
	{
		if cfg!(target_endian = "big")
		{
			NetworkByteOrderEndianU16(self.0 & 0x0FFF)
		}
		else
		{
			NetworkByteOrderEndianU16(self.0 & 0xFF0F)
		}
	}
}
