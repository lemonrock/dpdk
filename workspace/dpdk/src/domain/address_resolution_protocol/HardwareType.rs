// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Address resolution protocol (ARP) hardware type.
#[repr(C, packed)]
pub struct HardwareType(NetworkByteOrderEndianU16);

impl HardwareType
{
	/// Ethernet 2 (also known as Ethernet II).
	#[cfg(target_endian = "big")] pub const Ethernet2: Self = HardwareType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0001));
	
	/// Ethernet 2 (also known as Ethernet II).
	#[cfg(target_endian = "little")] pub const Ethernet2: Self = HardwareType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0100));
	
	/// Use this to eliminate unwanted ARP traffic.
	#[inline(always)]
	pub fn is_not_ethernet2(self) -> bool
	{
		self.0 != Self::Ethernet2.0
	}
}
