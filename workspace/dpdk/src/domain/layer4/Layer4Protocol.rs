// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layer4Protocol
{
	Tcp = 6,
	Udp = 17,
}

impl Layer4Protocol
{
	#[inline(always)]
	pub fn tldkValue(&self) -> u32
	{
		match *self
		{
			Layer4Protocol::Udp => ::dpdk_sys::TLE_PROTO_UDP,
			Layer4Protocol::Tcp => ::dpdk_sys::TLE_PROTO_TCP,
		}
	}
	
	#[inline(always)]
	pub fn libcValue(&self) -> u8
	{
		*self as u8
	}
}
