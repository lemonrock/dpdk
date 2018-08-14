// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Not all ethernet devices support all statistics.
///
/// Unfortunately, unsupported statistics are reported as zero (0) and aren't easily distinguished.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct EthernetPortSimpleStatistics
{
	/// Overview.
	pub overview: EthernetPortSimpleStatisticsOverview,
	
	/// Total number of unsuccessfully received packets.
	///
	/// (`rte_eth_stats.ierrors`).
	pub total_number_of_unsuccessfully_received_packets: u64,
	
	/// Total number of unsuccessfully transmitted packets.
	///
	/// (`rte_eth_stats.oerrors`).
	pub total_number_of_unsuccessfully_transmitted_packets: u64,
	
	/// Total number of packets received but for which a receive packet buffer (`rte_mbuf`) could not be allocated from the receive queue's memory pool.
	///
	/// A typical cause is being out-of-memory in the memory pool.
	///
	/// (`rte_eth_stats.rx_nombuf`).
	pub total_number_of_receive_allocation_failures: u64,
	
	/// For a QueueSimpleStatisticCounterIndex (0 to QueueSimpleStatisticCounterIndex::Maximum exclusive) associated with a queue, the total number of successfully received packets.
	///
	/// (`rte_eth_stats.q_ipackets`).
	total_number_of_successfully_received_packets_by_queue_counter: [u64; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	/// For a QueueSimpleStatisticCounterIndex (0 to QueueSimpleStatisticCounterIndex::Maximum exclusive) associated with a queue, the total number of successfully transmitted packets.
	///
	/// (`rte_eth_stats.q_opackets`).
	total_number_of_successfully_transmitted_packets_by_queue_counter: [u64; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	/// For a QueueSimpleStatisticCounterIndex (0 to QueueSimpleStatisticCounterIndex::Maximum exclusive) associated with a queue, the total number of successfully received bytes.
	///
	/// (`rte_eth_stats.q_ibytes`).
	total_number_of_successfully_received_bytes_by_queue_counter: [u64; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	/// For a QueueSimpleStatisticCounterIndex (0 to QueueSimpleStatisticCounterIndex::Maximum exclusive) associated with a queue, the total number of successfully transmitted bytes.
	///
	/// (`rte_eth_stats.q_obytes`).
	total_number_of_successfully_transmitted_bytes_by_queue_counter: [u64; QueueSimpleStatisticCounterIndex::Maximum as usize],
	
	/// For a QueueSimpleStatisticCounterIndex (0 to QueueSimpleStatisticCounterIndex::Maximum exclusive) associated with a queue, the total number of packets received but dropped before reaching software because there was no available received buffer.
	///
	/// Causes include there not being enough receive descriptors (ie the queue ring size aka queue depth was reached) and the received packet being larger, including headroom, than the receive queue's memory pool has been configured to support.
	///
	/// (`rte_eth_stats.q_errors`).
	total_number_of_packets_received_but_dropped_by_queue_counter: [u64; QueueSimpleStatisticCounterIndex::Maximum as usize],
}

impl From<rte_eth_stats> for EthernetPortSimpleStatistics
{
	#[inline(always)]
	fn from(value: rte_eth_stats) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl EthernetPortSimpleStatistics
{
	/// Simple statistics overflow for a queue (actually, a counter associated with a queue, for which there may not be a 1:1 mapping).
	#[inline(always)]
	pub fn overview_of_queue_statistics(&self, counter_index: QueueSimpleStatisticCounterIndex) -> EthernetPortSimpleStatisticsOverview
	{
		let counter_index: usize = counter_index.into();
		
		unsafe
		{
			EthernetPortSimpleStatisticsOverview
			{
				total_number_of_successfully_received_packets: *self.total_number_of_successfully_received_packets_by_queue_counter.get_unchecked(counter_index),
				total_number_of_successfully_transmitted_packets: *self.total_number_of_successfully_transmitted_packets_by_queue_counter.get_unchecked(counter_index),
				total_number_of_successfully_received_bytes: *self.total_number_of_successfully_received_bytes_by_queue_counter.get_unchecked(counter_index),
				total_number_of_successfully_transmitted_bytes: *self.total_number_of_successfully_transmitted_bytes_by_queue_counter.get_unchecked(counter_index),
				total_number_of_packets_received_but_dropped: *self.total_number_of_packets_received_but_dropped_by_queue_counter.get_unchecked(counter_index),
			}
		}
	}
}
