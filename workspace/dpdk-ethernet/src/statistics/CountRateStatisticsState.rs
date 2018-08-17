// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// State for count rate statistics.
///
/// Pass to `EthernetPortSimpleStatistics.update_count_rate_statistics()` to update.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct CountRateStatisticsState
{
	received_overview_packets: CountStatisticsState<PacketsCount>,
	received_overview_bytes: CountStatisticsState<BytesCount>,
	received_unsuccessfully_packets: CountStatisticsState<PacketsCount>,
	received_but_dropped_packets: CountStatisticsState<PacketsCount>,
	
	transmitted_overview_packets: CountStatisticsState<PacketsCount>,
	transmitted_overview_bytes: CountStatisticsState<BytesCount>,
	transmitted_unsuccessfully_packets: CountStatisticsState<PacketsCount>,
	
	received_allocation_failure_packets: CountStatisticsState<PacketsCount>,
	
	received_by_queue_counter_packets: [CountStatisticsState<PacketsCount>; QueueSimpleStatisticCounterIndex::Maximum as usize],
	received_by_queue_counter_bytes: [CountStatisticsState<BytesCount>; QueueSimpleStatisticCounterIndex::Maximum as usize],
	received_but_dropped_by_queue_counter_packets: [CountStatisticsState<PacketsCount>; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	transmitted_by_queue_counter_packets: [CountStatisticsState<PacketsCount>; QueueSimpleStatisticCounterIndex::Maximum as usize],
	transmitted_by_queue_counter_bytes: [CountStatisticsState<BytesCount>; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	/// Should default to either when the ethernet card was started, or the time statistics counters were last reset.
	sampled_at: MonotonicMillisecondTimestamp,
	
	/// A value of zero indicates infinity.
	interval: MillisecondDuration,
}

impl CountRateStatisticsState
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(when_ethernet_device_was_started_or_simple_statistics_last_reset: MonotonicMillisecondTimestamp) -> Self
	{
		Self
		{
			received_overview_packets: Default::default(),
			received_overview_bytes: Default::default(),
			received_unsuccessfully_packets: Default::default(),
			received_but_dropped_packets: Default::default(),
			
			transmitted_overview_packets: Default::default(),
			transmitted_overview_bytes: Default::default(),
			transmitted_unsuccessfully_packets: Default::default(),
			
			received_allocation_failure_packets: Default::default(),
			
			received_by_queue_counter_packets: Default::default(),
			received_by_queue_counter_bytes: Default::default(),
			received_but_dropped_by_queue_counter_packets: Default::default(),
			
			transmitted_by_queue_counter_packets: Default::default(),
			transmitted_by_queue_counter_bytes: Default::default(),
			
			sampled_at: when_ethernet_device_was_started_or_simple_statistics_last_reset,
			
			interval: CountRate::<BytesCount>::InfiniteInterval,
		}
	}
	
	#[inline(always)]
	pub(crate) fn calculate_rates(&mut self, ethernet_port_simple_statistics: &EthernetPortSimpleStatistics, sampled_at: MonotonicMillisecondTimestamp)
	{
		debug_assert!(sampled_at >= self.sampled_at, "Time has gone backwards");
		
		let overview = &ethernet_port_simple_statistics.overview;
		
		self.received_overview_packets.calculate_count_rates(overview.total_number_of_successfully_received_packets);
		self.received_overview_bytes.calculate_count_rates(overview.total_number_of_successfully_received_bytes);
		self.received_unsuccessfully_packets.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_unsuccessfully_received_packets);
		self.received_but_dropped_packets.calculate_count_rates(overview.total_number_of_packets_received_but_dropped);
		
		self.transmitted_overview_packets.calculate_count_rates(overview.total_number_of_successfully_transmitted_packets);
		self.transmitted_overview_bytes.calculate_count_rates(overview.total_number_of_successfully_transmitted_bytes);
		self.transmitted_unsuccessfully_packets.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_unsuccessfully_transmitted_packets);
		
		self.received_allocation_failure_packets.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_receive_allocation_failures);
		
		for queue_simple_statistic_counter_index in QueueSimpleStatisticCounterIndex::Zero ..= QueueSimpleStatisticCounterIndex::InclusiveMaximum
		{
			let received_by_queue_counter_packets = queue_simple_statistic_counter_index.get_mut(&mut self.received_by_queue_counter_packets);
			received_by_queue_counter_packets.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_received_packets_by_queue_counter(queue_simple_statistic_counter_index));
			let received_by_queue_counter_bytes = queue_simple_statistic_counter_index.get_mut(&mut self.received_by_queue_counter_bytes);
			received_by_queue_counter_bytes.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_received_bytes_by_queue_counter(queue_simple_statistic_counter_index));
			let received_but_dropped_by_queue_counter_packets = queue_simple_statistic_counter_index.get_mut(&mut self.received_but_dropped_by_queue_counter_packets);
			received_but_dropped_by_queue_counter_packets.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_packets_received_but_dropped_by_queue_counter(queue_simple_statistic_counter_index));
			
			let transmitted_by_queue_counter_packets = queue_simple_statistic_counter_index.get_mut(&mut self.transmitted_by_queue_counter_packets);
			transmitted_by_queue_counter_packets.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_transmitted_packets_by_queue_counter(queue_simple_statistic_counter_index));
			let transmitted_by_queue_counter_bytes = queue_simple_statistic_counter_index.get_mut(&mut self.transmitted_by_queue_counter_bytes);
			transmitted_by_queue_counter_bytes.calculate_count_rates(ethernet_port_simple_statistics.total_number_of_successfully_transmitted_bytes_by_queue_counter(queue_simple_statistic_counter_index));
		}
		
		self.updated_sample_timing_only_if_there_is_a_measurable_change_in_time(sampled_at);
	}
	
	/// When were these statistics sampled at?
	#[inline(always)]
	pub fn sampled_at(&self) -> MonotonicMillisecondTimestamp
	{
		self.sampled_at
	}
	
	/// Overview received (packets).
	#[inline(always)]
	pub fn received_overview_packets(&self) -> CountRateStatistics<PacketsCount>
	{
		self.received_overview_packets.count_rate_statistics(self.interval)
	}
	
	/// Overview received (bytes).
	#[inline(always)]
	pub fn received_overview_bytes(&self) -> CountRateStatistics<BytesCount>
	{
		self.received_overview_bytes.count_rate_statistics(self.interval)
	}
	
	/// Received unsuccessfully (packets only).
	#[inline(always)]
	pub fn received_unsuccessfully(&self) -> CountRateStatistics<PacketsCount>
	{
		self.received_unsuccessfully_packets.count_rate_statistics(self.interval)
	}
	
	/// Received but dropped (eg because the receive (ring) queue was full) (packets only and receive only; no transmit analog).
	#[inline(always)]
	pub fn received_but_dropped_packets(&self) -> CountRateStatistics<PacketsCount>
	{
		self.received_but_dropped_packets.count_rate_statistics(self.interval)
	}
	
	/// Received but allocation failures occurred (eg out of memory for packets) (packets only and receive only; no transmit or queue analog).
	#[inline(always)]
	pub fn received_allocation_failure_packets(&self) -> CountRateStatistics<PacketsCount>
	{
		self.received_allocation_failure_packets.count_rate_statistics(self.interval)
	}
	
	/// Overview transmitted (packets).
	#[inline(always)]
	pub fn transmitted_overview_packets(&self) -> CountRateStatistics<PacketsCount>
	{
		self.transmitted_overview_packets.count_rate_statistics(self.interval)
	}
	
	/// Overview transmitted (bytes).
	#[inline(always)]
	pub fn transmitted_overview_bytes(&self) -> CountRateStatistics<BytesCount>
	{
		self.transmitted_overview_bytes.count_rate_statistics(self.interval)
	}
	
	/// Transmitted unsuccessfully (packets only).
	#[inline(always)]
	pub fn transmitted_unsuccessfully(&self) -> CountRateStatistics<PacketsCount>
	{
		self.transmitted_unsuccessfully_packets.count_rate_statistics(self.interval)
	}
	
	/// Received by queue counter (packets).
	#[inline(always)]
	pub fn received_by_queue_counter_packets(&self, counter_index: QueueSimpleStatisticCounterIndex) -> CountRateStatistics<PacketsCount>
	{
		let counter_index: usize = counter_index.into();
		(unsafe { self.received_by_queue_counter_packets.get_unchecked(counter_index)}).count_rate_statistics(self.interval)
	}
	
	/// Received by queue counter (bytes).
	#[inline(always)]
	pub fn received_by_queue_counter_bytes(&self, counter_index: QueueSimpleStatisticCounterIndex) -> CountRateStatistics<BytesCount>
	{
		let counter_index: usize = counter_index.into();
		(unsafe { self.received_by_queue_counter_bytes.get_unchecked(counter_index)}).count_rate_statistics(self.interval)
	}
	
	/// Transmitted by queue counter (packets).
	#[inline(always)]
	pub fn transmitted_by_queue_counter_packets(&self, counter_index: QueueSimpleStatisticCounterIndex) -> CountRateStatistics<PacketsCount>
	{
		let counter_index: usize = counter_index.into();
		(unsafe { self.transmitted_by_queue_counter_packets.get_unchecked(counter_index)}).count_rate_statistics(self.interval)
	}
	
	/// Transmitted by queue counter (bytes).
	#[inline(always)]
	pub fn transmitted_by_queue_counter_bytes(&self, counter_index: QueueSimpleStatisticCounterIndex) -> CountRateStatistics<BytesCount>
	{
		let counter_index: usize = counter_index.into();
		(unsafe { self.transmitted_by_queue_counter_bytes.get_unchecked(counter_index)}).count_rate_statistics(self.interval)
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
