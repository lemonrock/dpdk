// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This type wraps an u16 such that it is in network byte order (big endian) form.
///
/// Internally, stores an u16 as a big endian bit pattern.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NetworkByteOrderEndianU16(u16);

impl NetworkByteOrderEndianU16
{
	/// Create from a network byte order value.
	///
	/// Assumes the value is already stored as a big endian bit pattern.
	#[inline(always)]
	pub fn from_network_byte_order_value(big_endian_value: u16) -> Self
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
	
	/// To native byte order.
	///
	/// On a Little Endian platform, will swap bytes.
	#[inline(always)]
	pub fn to_native_byte_order_value(self) -> u16
	{
		u16::from_be(self.0)
	}
}
