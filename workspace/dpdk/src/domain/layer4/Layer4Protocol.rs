// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents the Layer 4 protocol number.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layer4Protocol
{
	Tcp = 6,
	Udp = 17,
}

impl Layer4Protocol
{
	/// As the value used by TLDK.
	#[inline(always)]
	pub fn tldk_value(&self) -> u32
	{
		use self::Layer4Protocol::*;
		
		match *self
		{
			Udp => TLE_PROTO_UDP,
			Tcp => TLE_PROTO_TCP,
		}
	}
	
	/// As the value used by libc.
	#[inline(always)]
	pub fn libc_value(&self) -> u8
	{
		*self as u8
	}
}
