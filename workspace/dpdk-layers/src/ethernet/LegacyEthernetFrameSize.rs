// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Legacy ethernet frame size.
///
/// Rare and of little value for typical DPDK usage.
#[repr(C, packed)]
#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LegacyEthernetFrameSize(NetworkByteOrderEndianU16);

impl Into<NetworkByteOrderEndianU16> for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn into(self) -> NetworkByteOrderEndianU16
	{
		self.0
	}
}

impl Into<u16> for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_byte_order_value()
	}
}

impl From<NetworkByteOrderEndianU16> for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn from(value: NetworkByteOrderEndianU16) -> Self
	{
		LegacyEthernetFrameSize(value)
	}
}

impl From<u16> for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		LegacyEthernetFrameSize(NetworkByteOrderEndianU16::from_native_byte_order_value(value))
	}
}

impl Display for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_byte_order_value())
	}
}

impl Debug for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_byte_order_value())
	}
}

impl LegacyEthernetFrameSize
{
	pub const Minimum: Self = LegacyEthernetFrameSize(NetworkByteOrderEndianU16::from_network_byte_order_value(0x05FF));
	
	/// IEEE 802.3x-1997 frame size change over.
	#[cfg(target_endian = "big")] pub const Maximum: Self = LegacyEthernetFrameSize(NetworkByteOrderEndianU16::from_network_byte_order_value(0x05FF));
	
	/// IEEE 802.3x-1997 frame size change over.
	#[cfg(target_endian = "little")] pub const Maximum: Self = LegacyEthernetFrameSize(NetworkByteOrderEndianU16::from_network_byte_order_value(0xFF05));
}
