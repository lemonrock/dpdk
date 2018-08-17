// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An interval of `Self::InfiniteInterval` (zero) is infinite.
///
/// A rate is `count / interval`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct CountRate<C: Count>
{
	/// Count.
	pub count: C,
	
	/// Interval.
	pub interval: MillisecondDuration,
}

impl<C: Count> CountRate<C>
{
	/// An infinite interval.
	pub const InfiniteInterval: MillisecondDuration = MillisecondDuration::Zero;
	
	#[inline(always)]
	pub(crate) const fn new(count: C, interval: MillisecondDuration) -> Self
	{
		Self
		{
			count,
			interval,
		}
	}
	
	/// Rate per millisecond, rounded down.
	#[inline(always)]
	pub fn rate_per_millisecond_rounded_down(&self) -> Option<u64>
	{
		let count: u64 = self.count.into();
		let interval: u64 = self.interval.into();
		count.checked_div(interval)
	}
	
	/// Rate per second, rounded down.
	#[inline(always)]
	pub fn rate_per_second_rounded_down(&self) -> Option<u64>
	{
		let count: u64 = self.count.into();
		let interval: u64 = self.interval.into();
		count.checked_div(interval / 1000)
	}
}
