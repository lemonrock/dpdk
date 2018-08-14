// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a Linux or FreeBSD interface index (a value that maps to things like `eth0`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct InterfaceIndex(NonZeroU32);

impl From<NonZeroU32> for InterfaceIndex
{
	#[inline(always)]
	fn from(value: NonZeroU32) -> Self
	{
		InterfaceIndex(value)
	}
}

impl Into<NonZeroU32> for InterfaceIndex
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		self.0
	}
}

impl Into<u32> for InterfaceIndex
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0.get()
	}
}
