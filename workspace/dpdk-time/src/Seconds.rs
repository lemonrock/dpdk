// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Seconds.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Seconds(u64);

impl From<u8> for Seconds
{
	#[inline(always)]
	fn from(seconds: u8) -> Self
	{
		Seconds(seconds as u64)
	}
}

impl From<u16> for Seconds
{
	#[inline(always)]
	fn from(seconds: u16) -> Self
	{
		Seconds(seconds as u64)
	}
}

impl From<u32> for Seconds
{
	#[inline(always)]
	fn from(seconds: u32) -> Self
	{
		Seconds(seconds as u64)
	}
}

impl From<usize> for Seconds
{
	#[inline(always)]
	fn from(seconds: usize) -> Self
	{
		Seconds(seconds as u64)
	}
}

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
	
	/// Zero.
	pub const Zero: Self = Seconds(0);
	
	/// Thirty seconds.
	pub const ThirtySeconds: Self = Seconds(30);
	
	/// Two minutes.
	pub const TwoMinutes: Self = Seconds(120);
}
