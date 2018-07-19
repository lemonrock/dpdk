// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a RFC 1071 internet checksum.
#[repr(C, packed)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternetChecksum(NetworkByteOrderEndianU16);

impl Into<NetworkByteOrderEndianU16> for InternetChecksum
{
	#[inline(always)]
	fn into(self) -> NetworkByteOrderEndianU16
	{
		self.0
	}
}

impl Into<u16> for InternetChecksum
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_byte_order_value()
	}
}

impl From<NetworkByteOrderEndianU16> for InternetChecksum
{
	#[inline(always)]
	fn from(value: NetworkByteOrderEndianU16) -> Self
	{
		InternetChecksum(value)
	}
}

impl From<u16> for InternetChecksum
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		InternetChecksum(NetworkByteOrderEndianU16::from_native_byte_order_value(value))
	}
}

impl Display for InternetChecksum
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_byte_order_value())
	}
}

impl Debug for InternetChecksum
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_byte_order_value())
	}
}

impl Default for InternetChecksum
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetChecksum(NetworkByteOrderEndianU16::Zero)
	}
}
