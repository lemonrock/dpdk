// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Seconds.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Seconds(pub u64);

impl From<u64> for Seconds
{
	#[inline(always)]
	fn from(seconds: u64) -> Self
	{
		Seconds(seconds)
	}
}

impl Into<u64> for Seconds
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Seconds
{
	/// Is zero?
	#[inline(always)]
	pub fn is_zero(self) -> bool
	{
		self.0 == 0
	}
}
