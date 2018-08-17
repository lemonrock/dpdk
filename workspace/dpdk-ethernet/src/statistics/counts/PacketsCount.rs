// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a counter of packets received, transmitted, dropped, etc.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct PacketsCount(pub u64);

impl From<u64> for PacketsCount
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		PacketsCount(value)
	}
}

impl From<i64> for PacketsCount
{
	#[inline(always)]
	fn from(value: i64) -> Self
	{
		PacketsCount(value as u64)
	}
}

impl Into<u64> for PacketsCount
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Into<i64> for PacketsCount
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self.0 as i64
	}
}

impl Display for PacketsCount
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(&self.0, f)
	}
}

impl Sub for PacketsCount
{
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output
	{
		PacketsCount(self.0 - rhs.0)
	}
}

impl Count for PacketsCount
{
	const ZeroOrSimpleStatisticNotSupportedByEthernetDevice: Self = PacketsCount(0);
	
	#[inline(always)]
	fn is_zero(self) -> bool
	{
		self.0 == 0
	}
	
	#[inline(always)]
	fn calculate_count_rates(count_rate_statistics_state: &mut CountRateStatisticsState<Self>, ethernet_port_simple_statistics: &EthernetPortSimpleStatistics, sampled_at: MonotonicMillisecondTimestamp)
	{
		count_rate_statistics_state.calculate_count_rates(ethernet_port_simple_statistics, sampled_at)
	}
}
