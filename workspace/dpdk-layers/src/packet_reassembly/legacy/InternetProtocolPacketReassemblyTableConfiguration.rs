// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolPacketReassemblyTableConfiguration
{
	#[serde(default = "InternetProtocolPacketReassemblyTableConfiguration::maximum_number_of_packets_being_reassembled_at_any_one_time_default")] maximum_number_of_packets_being_reassembled_at_any_one_time: u16,
	#[serde(default = "InternetProtocolPacketReassemblyTableConfiguration::entries_per_bucket_default")] entries_per_bucket: u16,
	#[serde(default = "InternetProtocolPacketReassemblyTableConfiguration::reassembly_timeout_default")] reassembly_timeout: Seconds,
}

impl Display for InternetProtocolPacketReassemblyTableConfiguration
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Default for InternetProtocolPacketReassemblyTableConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			maximum_number_of_packets_being_reassembled_at_any_one_time: Self::maximum_number_of_packets_being_reassembled_at_any_one_time_default(),
			entries_per_bucket: Self::entries_per_bucket_default(),
			reassembly_timeout: Self::reassembly_timeout_default(),
		}
	}
}

impl InternetProtocolPacketReassemblyTableConfiguration
{
	/// Creates a new internet protocol (IP) packet reassembly table.
	pub fn create_table(&self, numa_node_choice: NumaNodeChoice) -> Result<UnsafeCell<InternetProtocolPacketReassemblyTable>, ()>
	{
		InternetProtocolPacketReassemblyTable::create(self.maximum_number_of_packets_being_reassembled_at_any_one_time, self.entries_per_bucket, self.reassembly_timeout, numa_node_choice).map(|table| UnsafeCell::new(table))
	}
	
	#[inline(always)]
	const fn maximum_number_of_packets_being_reassembled_at_any_one_time_default() -> u16
	{
		4094
	}
	
	#[inline(always)]
	const fn entries_per_bucket_default() -> u16
	{
		16
	}
	
	#[inline(always)]
	const fn reassembly_timeout_default() -> Seconds
	{
		Seconds::ThirtySeconds
	}
}
