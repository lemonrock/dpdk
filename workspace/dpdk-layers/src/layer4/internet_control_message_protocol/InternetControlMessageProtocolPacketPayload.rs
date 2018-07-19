// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
///
/// See RFC 792.
///
/// Example: Destination Unreachable: Internet Header + 64 bits of Original Data Datagram
#[repr(C, packed)]
pub union InternetControlMessageProtocolPacketPayload
{
	pub other: PhantomData<u8>,
}

impl Display for InternetControlMessageProtocolPacketPayload
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for InternetControlMessageProtocolPacketPayload
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(ICMP payload)")
	}
}
