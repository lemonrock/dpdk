// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents bit statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
struct BitStatistics
{
	/// Peak bits.
	///
	/// Divide by the time taken since the last sample of statistics was made to get the peak bit rate.
	peak_bits: u64,
	
	/// Unsmoothed mean for just the current time delta.
	///
	/// Divide by the time taken since the last sample of statistics was made to get the unsmoothed mean bit rate.
	unsmoothed_mean_bits: u64,
	
	/// An iteratively calculated Exponentially Weighted Moving Average (EWMA) that uses a weighting factor of `AlphaPercent` (currently 20%).
	///
	/// Divide by the time taken since the last sample of statistics was made to get the  Exponentially Weighted Moving Average bit rate.
	exponentionally_weighted_moving_average_bits: u64,
}

impl BitStatistics
{
	#[inline(always)]
	fn peak_bit_rate(&self, interval: MillisecondDuration) -> BitRate
	{
		BitRate::new(self.peak_bits, interval)
	}
	
	#[inline(always)]
	fn unsmoothed_mean_bit_rate(&self, interval: MillisecondDuration) -> BitRate
	{
		BitRate::new(self.unsmoothed_mean_bits, interval)
	}
	
	#[inline(always)]
	fn exponentionally_weighted_moving_average_bit_rate(&self, interval: MillisecondDuration) -> BitRate
	{
		BitRate::new(self.exponentionally_weighted_moving_average_bits, interval)
	}
	
	#[inline(always)]
	pub(crate) fn update_peak(&mut self, interval_number_of_bits: u64)
	{
		if interval_number_of_bits > self.peak_bits
		{
			self.peak_bits = interval_number_of_bits;
		}
	}
	
	#[inline(always)]
	pub(crate) fn set_unsmoothed_mean(&mut self, interval_number_of_bits: u64)
	{
		self.unsmoothed_mean_bits = interval_number_of_bits;
	}
	
	#[inline(always)]
	pub(crate) fn adjust_exponentionally_weighted_moving_average_bits(&mut self, interval_number_of_bits: u64)
	{
		let exponentionally_weighted_moving_average_bits = self.exponentionally_weighted_moving_average_bits as i64;
		
		let delta = interval_number_of_bits as i64 - exponentionally_weighted_moving_average_bits;
		
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
		if interval_number_of_bits == 0 && increment == 0
		{
			self.exponentionally_weighted_moving_average_bits = 0;
		}
		else
		{
			self.exponentionally_weighted_moving_average_bits = (exponentionally_weighted_moving_average_bits + increment) as u64;
		}
	}
}
