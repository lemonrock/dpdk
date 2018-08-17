// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a counter of bytes received or transmitted.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BytesCounter(pub u64);

impl From<u64> for BytesCounter
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		BytesCounter(value)
	}
}

impl Into<u64> for BytesCounter
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Display for BytesCounter
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(&self.0, f)
	}
}

impl Sub for BytesCounter
{
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output
	{
		BytesCounter(self.0 - rhs.0)
	}
}

impl BytesCounter
{
	/// Some ethernet devices do not support some simple statistics; they record these as zero, rather than use a sentinel or Option.
	pub const ZeroOrSimpleStatisticNotSupportedByEthernetDevice: BytesCounter = BytesCounter(0);
	
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
