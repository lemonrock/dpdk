// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Simple statistics overview.
///
/// Not all ethernet devices support all statistics.
///
/// Unfortunately, unsupported statistics are reported as zero (0) and aren't easily distinguished.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct EthernetPortSimpleStatisticsOverview
{
	/// Total number of successfully received packets.
	///
	/// (`rte_eth_stats.ipackets`).
	pub total_number_of_successfully_received_packets: u64,
	
	/// Total number of successfully transmitted packets.
	///
	/// (`rte_eth_stats.opackets`).
	pub total_number_of_successfully_transmitted_packets: u64,
	
	/// Total number of successfully received packets.
	///
	/// (`rte_eth_stats.ibytes`).
	pub total_number_of_successfully_received_bytes: u64,
	
	/// Total number of successfully transmitted packets.
	///
	/// (`rte_eth_stats.obytes`).
	pub total_number_of_successfully_transmitted_bytes: u64,
	
	/// Total number of packets received but dropped before reaching software because there was no available received buffer.
	///
	/// Causes include there not being enough receive descriptors (ie the queue ring size aka queue depth was reached) and the received packet being larger, including headroom, than the receive queue's memory pool has been configured to support.
	///
	/// (`rte_eth_stats.imissed`).
	pub total_number_of_packets_received_but_dropped: u64,
}
