// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Transmit and receive queues can calculate statistics.
///
/// These are stored in a fixed range of statistic counters (`Self::Maximum`, usually 16), indexed by this struct.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct QueueSimpleStatisticCounterIndex(u8);

impl TryFrom<u8> for QueueSimpleStatisticCounterIndex
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value < Self::Maximum
		{
			Ok(QueueSimpleStatisticCounterIndex(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Into<usize> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl QueueSimpleStatisticCounterIndex
{
	/// Maximum queue statistic counters.
	pub const Maximum: u8 = RTE_ETHDEV_QUEUE_STAT_CNTRS as u8;
}
