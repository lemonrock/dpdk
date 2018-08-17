// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents count rate statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct CountRateStatistics<C: Count>
{
	/// Peak.
	///
	/// Divide by the time taken since the last sample of statistics was made to get the peak count rate.
	pub peak: CountRate<C>,
	
	/// Unsmoothed mean for just the current time delta.
	///
	/// Divide by the time taken since the last sample of statistics was made to get the unsmoothed mean count rate.
	pub unsmoothed_mean: CountRate<C>,
	
	/// An iteratively calculated Exponentially Weighted Moving Average (EWMA) that uses a weighting factor of `AlphaPercent` (currently 20%).
	///
	/// Divide by the time taken since the last sample of statistics was made to get the Exponentially Weighted Moving Average count rate.
	pub exponentionally_weighted_moving_average: CountRate<C>,
}

impl<C: Count> CountRateStatistics<C>
{
	#[inline(always)]
	fn new(statistics: &CountStatistics<C>, interval: MillisecondDuration) -> Self
	{
		Self
		{
			peak: statistics.peak_rate(interval),
			unsmoothed_mean: statistics.unsmoothed_mean_rate(interval),
			exponentionally_weighted_moving_average: statistics.exponentionally_weighted_moving_average_rate(interval),
		}
	}
}
