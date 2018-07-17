// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub union Layer4Packet
{
	/// Internet Control Message Protocol (ICMP).
	pub icmp: InternetControlMessageProtocolType,
	
	/// Transmission Control Protocol (TCP).
	pub tcp: (),
	
	/// Unreliable Datagram Protocol (UDP).
	pub udp: (),
}
