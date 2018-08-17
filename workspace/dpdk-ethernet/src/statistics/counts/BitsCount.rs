// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a count of bits received, transmitted, etc.
///
/// Use `From / Into` impls to convert to or from a BytesCount.
///
/// Conversions round down, so aren't communitative.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BitsCount(pub u64);

impl From<u64> for BitsCount
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		BitsCount(value)
	}
}

impl From<i64> for BitsCount
{
	#[inline(always)]
	fn from(value: i64) -> Self
	{
		BitsCount(value as u64)
	}
}

impl From<BytesCount> for BitsCount
{
	#[inline(always)]
	fn from(value: BytesCount) -> Self
	{
		const BitsPerBytes: u64 = 8;
		
		BitsCount(value.0 * BitsPerBytes)
	}
}

impl Into<u64> for BitsCount
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Into<i64> for BitsCount
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self.0 as i64
	}
}

impl Display for BitsCount
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(&self.0, f)
	}
}

impl Sub for BitsCount
{
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output
	{
		BitsCount(self.0 - rhs.0)
	}
}

impl Count for BitsCount
{
	const ZeroOrSimpleStatisticNotSupportedByEthernetDevice: Self = BitsCount(0);
	
	#[inline(always)]
	fn is_zero(self) -> bool
	{
		self.0 == 0
	}
}
