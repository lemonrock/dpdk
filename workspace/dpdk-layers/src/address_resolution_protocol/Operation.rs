// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Address resolution protocol (ARP) hardware type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C, packed)]
pub struct Operation(NetworkByteOrderEndianU16);

impl Operation
{
	/// Request.
	#[cfg(target_endian = "big")] pub const Request: Self = Operation(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0001));
	
	/// Request.
	#[cfg(target_endian = "little")] pub const Request: Self = Operation(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0100));
	
	/// Reply.
	#[cfg(target_endian = "big")] pub const Reply: Self = Operation(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0002));
	
	/// Reply.
	#[cfg(target_endian = "little")] pub const Reply: Self = Operation(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0200));
}
