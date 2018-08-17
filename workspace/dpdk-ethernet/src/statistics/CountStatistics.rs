// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents bit statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
struct CountStatistics<C: Count>
{
	/// Peak count.
	///
	/// Divide by the time taken since the last sample of statistics was made to get the peak count rate.
	peak_count: C,
	
	/// Unsmoothed mean for just the current time delta.
	///
	/// Divide by the time taken since the last sample of statistics was made to get the unsmoothed mean count rate.
	unsmoothed_mean_count: C,
	
	/// An iteratively calculated Exponentially Weighted Moving Average (EWMA) that uses a weighting factor of `AlphaPercent` (currently 20%).
	///
	/// Divide by the time taken since the last sample of statistics was made to get the  Exponentially Weighted Moving Average count rate.
	exponentionally_weighted_moving_average_count: C,
}

impl<C: Count> CountStatistics<C>
{
	#[inline(always)]
	fn peak_rate(&self, interval: MillisecondDuration) -> CountRate<C>
	{
		CountRate::new(self.peak_count, interval)
	}
	
	#[inline(always)]
	fn unsmoothed_mean_rate(&self, interval: MillisecondDuration) -> CountRate<C>
	{
		CountRate::new(self.unsmoothed_mean_count, interval)
	}
	
	#[inline(always)]
	fn exponentionally_weighted_moving_average_rate(&self, interval: MillisecondDuration) -> CountRate<C>
	{
		CountRate::new(self.exponentionally_weighted_moving_average_count, interval)
	}
	
	#[inline(always)]
	pub(crate) fn update_peak(&mut self, interval_count: C)
	{
		if interval_count > self.peak_count
		{
			self.peak_count = interval_count;
		}
	}
	
	#[inline(always)]
	pub(crate) fn set_unsmoothed_mean(&mut self, interval_count: C)
	{
		self.unsmoothed_mean_count = interval_count;
	}
	
	#[inline(always)]
	pub(crate) fn adjust_exponentionally_weighted_moving_average_bits(&mut self, interval_count: C)
	{
		let exponentionally_weighted_moving_average_bits: i64 = self.exponentionally_weighted_moving_average_count.into();
		
		let delta: i64 = { let into: i64 = interval_count.into(); into } - exponentionally_weighted_moving_average_bits;
		
		// The +50 / -50 fixes integer rounding (down) during division.
		const AlphaPercent: i64 = 20;
		let increment = if delta > 0
		{
			(delta * AlphaPercent + 50) / 100
		}
		else
		{
			(delta * AlphaPercent - 50) / 100
		};
		
		// Integer rounding (down) above prevents the Exponentially Weighted Moving Average between 0 and `100 / AlphaPercent` ever reaching zero when there is no traffic in an interval.
		if interval_count.is_zero() && increment == 0
		{
			self.exponentionally_weighted_moving_average_count = C::ZeroOrSimpleStatisticNotSupportedByEthernetDevice;
		}
		else
		{
			self.exponentionally_weighted_moving_average_count = C::from(exponentionally_weighted_moving_average_bits + increment);
		}
	}
}
