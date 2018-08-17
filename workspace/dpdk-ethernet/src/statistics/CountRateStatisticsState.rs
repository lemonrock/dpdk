// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// State for bit rate statistics.
///
/// Pass to `EthernetPortSimpleStatistics.update_count_rate_statistics()` to update.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct CountRateStatisticsState<C: Count>
{
	received_overview: CountStatisticsState<C>,
	
	transmitted_overview: CountStatisticsState<C>,
	
	received_by_queue_counter: [CountStatisticsState<C>; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	transmitted_by_queue_counter: [CountStatisticsState<C>; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	/// Should default to either when the ethernet card was started, or the time statistics counters were last reset.
	sampled_at: MonotonicMillisecondTimestamp,
	
	/// A value of zero indicates infinity.
	interval: MillisecondDuration,
}

impl CountRateStatisticsState<PacketsCount>
{
	#[inline(always)]
	pub(crate) fn calculate_count_rates(&mut self, ethernet_port_simple_statistics: &EthernetPortSimpleStatistics, sampled_at: MonotonicMillisecondTimestamp)
	{
		debug_assert!(sampled_at >= self.sampled_at, "Time has gone backwards");
		
		let overview = &ethernet_port_simple_statistics.overview;
		self.received_overview.calculate_count_rates(overview.total_number_of_successfully_received_packets);
		self.transmitted_overview.calculate_count_rates(overview.total_number_of_successfully_transmitted_packets);
		
		for queue_simple_statistic_counter_index in QueueSimpleStatisticCounterIndex::Zero ..= QueueSimpleStatisticCounterIndex::InclusiveMaximum
		{
			let received_by_queue_counter = queue_simple_statistic_counter_index.get_mut(&mut self.received_by_queue_counter);
			received_by_queue_counter.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_received_packets_by_queue_counter(queue_simple_statistic_counter_index));
			
			let transmitted_by_queue_counter = queue_simple_statistic_counter_index.get_mut(&mut self.transmitted_by_queue_counter);
			transmitted_by_queue_counter.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_transmitted_packets_by_queue_counter(queue_simple_statistic_counter_index));
		}
		
		self.updated_sample_timing_only_if_there_is_a_measurable_change_in_time(sampled_at);
	}
}

impl CountRateStatisticsState<BytesCount>
{
	#[inline(always)]
	pub(crate) fn calculate_count_rates(&mut self, ethernet_port_simple_statistics: &EthernetPortSimpleStatistics, sampled_at: MonotonicMillisecondTimestamp)
	{
		debug_assert!(sampled_at >= self.sampled_at, "Time has gone backwards");
		
		let overview = &ethernet_port_simple_statistics.overview;
		self.received_overview.calculate_count_rates(overview.total_number_of_successfully_received_bytes);
		self.transmitted_overview.calculate_count_rates(overview.total_number_of_successfully_transmitted_bytes);
		
		for queue_simple_statistic_counter_index in QueueSimpleStatisticCounterIndex::Zero ..= QueueSimpleStatisticCounterIndex::InclusiveMaximum
		{
			let received_by_queue_counter = queue_simple_statistic_counter_index.get_mut(&mut self.received_by_queue_counter);
			received_by_queue_counter.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_received_bytes_by_queue_counter(queue_simple_statistic_counter_index));
			
			let transmitted_by_queue_counter = queue_simple_statistic_counter_index.get_mut(&mut self.transmitted_by_queue_counter);
			transmitted_by_queue_counter.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_transmitted_bytes_by_queue_counter(queue_simple_statistic_counter_index));
		}
		
		self.updated_sample_timing_only_if_there_is_a_measurable_change_in_time(sampled_at);
	}
}

impl<C: Count> CountRateStatisticsState<C>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(when_ethernet_device_was_started_or_simple_statistics_last_reset: MonotonicMillisecondTimestamp) -> Self
	{
		Self
		{
			received_overview: Default::default(),
			
			transmitted_overview: Default::default(),
			
			received_by_queue_counter: Default::default(),
			
			transmitted_by_queue_counter: Default::default(),
			
			sampled_at: when_ethernet_device_was_started_or_simple_statistics_last_reset,
			
			interval: CountRate::<C>::InfiniteInterval,
		}
	}
	
	/// When were these statistics sampled at?
	#[inline(always)]
	pub fn sampled_at(&self) -> MonotonicMillisecondTimestamp
	{
		self.sampled_at
	}
	
	/// Overview received.
	#[inline(always)]
	pub fn overview_received(&self) -> CountRateStatistics<C>
	{
		self.received_overview.count_rate_statistics(self.interval)
	}
	
	/// Overview transmitted.
	#[inline(always)]
	pub fn overview_transmitted(&self) -> CountRateStatistics<C>
	{
		self.transmitted_overview.count_rate_statistics(self.interval)
	}
	
	/// Received by queue counter.
	#[inline(always)]
	pub fn received_by_queue_counter(&self, counter_index: QueueSimpleStatisticCounterIndex) -> CountRateStatistics<C>
	{
		let counter_index: usize = counter_index.into();
		(unsafe { self.received_by_queue_counter.get_unchecked(counter_index)}).count_rate_statistics(self.interval)
	}
	
	/// Transmitted by queue counter.
	#[inline(always)]
	pub fn transmitted_by_queue_counter(&self, counter_index: QueueSimpleStatisticCounterIndex) -> CountRateStatistics<C>
	{
		let counter_index: usize = counter_index.into();
		(unsafe { self.transmitted_by_queue_counter.get_unchecked(counter_index)}).count_rate_statistics(self.interval)
	}
	
	#[inline(always)]
	fn updated_sample_timing_only_if_there_is_a_measurable_change_in_time(&mut self, sampled_at: MonotonicMillisecondTimestamp)
	{
		if likely!(sampled_at > self.sampled_at)
		{
			self.interval = sampled_at - self.sampled_at;
			self.sampled_at = sampled_at;
		}
	}
}
