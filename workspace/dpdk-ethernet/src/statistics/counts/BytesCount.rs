// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a counter of bytes received or transmitted.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BytesCount(pub u64);

impl From<u64> for BytesCount
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		BytesCount(value)
	}
}

impl From<i64> for BytesCount
{
	#[inline(always)]
	fn from(value: i64) -> Self
	{
		BytesCount(value as u64)
	}
}

impl Into<u64> for BytesCount
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Into<i64> for BytesCount
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self.0 as i64
	}
}

impl Display for BytesCount
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(&self.0, f)
	}
}

impl Sub for BytesCount
{
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output
	{
		BytesCount(self.0 - rhs.0)
	}
}

impl Count for BytesCount
{
	const ZeroOrSimpleStatisticNotSupportedByEthernetDevice: Self = BytesCount(0);
	
	#[inline(always)]
	fn is_zero(self) -> bool
	{
		self.0 == 0
	}
}

impl BytesCount
{
	/// To a bits value.
	///
	/// Incorrect if the number of bytes >= 2 ^ 61.
	#[inline(always)]
	pub fn to_bits(self) -> u64
	{
		const BitsPerBytes: u64 = 8;
		
		self.0 * BitsPerBytes
	}
}
