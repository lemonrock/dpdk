// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An interval of `Self::InfiniteInterval` (zero) is infinite.
///
/// A bit rate is `bits / interval`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct BitRate
{
	/// Bits.
	pub bits: u64,
	
	/// Interval.
	pub interval: MillisecondDuration,
}

impl BitRate
{
	/// An infinite interval.
	pub const InfiniteInterval: MillisecondDuration = MillisecondDuration::Zero;
	
	#[inline(always)]
	pub(crate) const fn new(bits: u64, interval: MillisecondDuration) -> Self
	{
		Self
		{
			bits,
			interval,
		}
	}
	
	/// Rate in bits per millisecond, rounded down.
	#[inline(always)]
	pub fn rate_in_bits_per_millisecond_rounded_down(&self) -> Option<u64>
	{
		let into: u64 = self.interval.into();
		self.bits.checked_div(into)
	}
	
	/// Rate in bits per second, rounded down.
	#[inline(always)]
	pub fn rate_in_bits_per_second_rounded_down(&self) -> Option<u64>
	{
		let into: u64 = self.interval.into();
		self.bits.checked_div(into / 1000)
	}
}
