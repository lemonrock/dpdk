// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// One instance per ethernet port.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
struct CountStatisticsState<C: Count>
{
	previous_total_number_of_count: C,
	statistics: CountStatistics<C>,
}

impl<C: Count> CountStatisticsState<C>
{
	#[inline(always)]
	pub(crate) fn calculate_count_rates(&mut self, total_number_of_count: C)
	{
		let interval_count = self.interval_count_and_record_previous_total_number_of_count(total_number_of_count);
		
		let statistics = &mut self.statistics;
		statistics.update_peak(interval_count);
		statistics.set_unsmoothed_mean(interval_count);
		statistics.adjust_exponentionally_weighted_moving_average_count(interval_count)
	}
	
	#[inline(always)]
	pub(crate) fn count_rate_statistics(&self, interval: MillisecondDuration) -> CountRateStatistics<C>
	{
		CountRateStatistics::new(&self.statistics, interval)
	}
	
	#[inline(always)]
	fn interval_count_and_record_previous_total_number_of_count(&mut self, total_number_of_count: C) -> C
	{
		let previous_total_number_of_count = self.previous_total_number_of_count;
		
		let statistics_counter_was_reset = total_number_of_count < previous_total_number_of_count;
		if unlikely!(statistics_counter_was_reset)
		{
			self.previous_total_number_of_count = C::ZeroOrSimpleStatisticNotSupportedByEthernetDevice;
			total_number_of_count
		}
		else
		{
			self.previous_total_number_of_count = total_number_of_count;
			(total_number_of_count - previous_total_number_of_count)
		}
	}
}
