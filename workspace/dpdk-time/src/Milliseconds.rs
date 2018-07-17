// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Milliseconds.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Milliseconds(u64);

impl From<u8> for Milliseconds
{
	#[inline(always)]
	fn from(milliseconds: u8) -> Self
	{
		Milliseconds(milliseconds as u64)
	}
}

impl From<u16> for Milliseconds
{
	#[inline(always)]
	fn from(milliseconds: u16) -> Self
	{
		Milliseconds(milliseconds as u64)
	}
}

impl From<u32> for Milliseconds
{
	#[inline(always)]
	fn from(milliseconds: u32) -> Self
	{
		Milliseconds(milliseconds as u64)
	}
}

impl From<usize> for Milliseconds
{
	#[inline(always)]
	fn from(milliseconds: usize) -> Self
	{
		Milliseconds(milliseconds as u64)
	}
}

impl From<u64> for Milliseconds
{
	#[inline(always)]
	fn from(milliseconds: u64) -> Self
	{
		Milliseconds(milliseconds)
	}
}

impl Into<u64> for Milliseconds
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Milliseconds
{
	/// Wait at lest this number of milliseconds.
	#[inline(always)]
	pub fn wait_at_least(self)
	{
		unsafe { rust_rte_delay_ms(self.0 as u32) }
	}
}
