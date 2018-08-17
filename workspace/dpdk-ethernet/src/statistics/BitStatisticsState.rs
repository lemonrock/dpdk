// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// One instance per ethernet port.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
struct BitStatisticsState
{
	previous_total_number_of_bytes: BytesCounter,
	statistics: BitStatistics,
}

impl BitStatisticsState
{
	#[inline(always)]
	pub(crate) fn calculate_bit_rates(&mut self, total_number_of_bytes: BytesCounter)
	{
		let interval_number_of_bits = self.interval_number_of_bits_and_record_previous_total_number_of_bytes(total_number_of_bytes);
		
		let statistics = &mut self.statistics;
		statistics.update_peak(interval_number_of_bits);
		statistics.set_unsmoothed_mean(interval_number_of_bits);
		statistics.adjust_exponentionally_weighted_moving_average_bits(interval_number_of_bits)
	}
	
	#[inline(always)]
	pub(crate) fn bit_rate_statistics(&self, interval: MillisecondDuration) -> BitRateStatistics
	{
		BitRateStatistics::new(&self.statistics, interval)
	}
	
	#[inline(always)]
	fn interval_number_of_bits_and_record_previous_total_number_of_bytes(&mut self, total_number_of_bytes: BytesCounter) -> u64
	{
		let previous_total_number_of_bytes = self.previous_total_number_of_bytes;
		
		let statistics_counter_was_reset = total_number_of_bytes < previous_total_number_of_bytes;
		let previous_total_number_of_bytes = if unlikely!(statistics_counter_was_reset)
		{
			BytesCounter::ZeroOrSimpleStatisticNotSupportedByEthernetDevice
		}
		else
		{
			previous_total_number_of_bytes
		};
		
		let interval_number_of_bits = (total_number_of_bytes - previous_total_number_of_bytes).to_bits();
		self.previous_total_number_of_bytes = total_number_of_bytes;
		interval_number_of_bits
	}
}
