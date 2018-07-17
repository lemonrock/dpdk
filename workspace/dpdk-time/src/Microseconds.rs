// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Microseconds.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Microseconds(u64);

impl From<u8> for Microseconds
{
	#[inline(always)]
	fn from(microseconds: u8) -> Self
	{
		Microseconds(microseconds as u64)
	}
}

impl From<u16> for Microseconds
{
	#[inline(always)]
	fn from(microseconds: u16) -> Self
	{
		Microseconds(microseconds as u64)
	}
}

impl From<u32> for Microseconds
{
	#[inline(always)]
	fn from(microseconds: u32) -> Self
	{
		Microseconds(microseconds as u64)
	}
}

impl From<usize> for Microseconds
{
	#[inline(always)]
	fn from(microseconds: usize) -> Self
	{
		Microseconds(microseconds as u64)
	}
}

impl From<u64> for Microseconds
{
	#[inline(always)]
	fn from(microseconds: u64) -> Self
	{
		Microseconds(microseconds)
	}
}

impl Into<u64> for Microseconds
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Microseconds
{
	/// Wait at lest this number of microseconds.
	#[inline(always)]
	pub fn wait_at_least(self)
	{
		unsafe { rte_delay_us_block(self.0 as u32) }
	}
}
