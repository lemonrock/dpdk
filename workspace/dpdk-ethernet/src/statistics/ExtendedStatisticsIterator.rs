// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents an iterator over extended statistics.
///
/// Obtain using `EthernetDeviceCapabilities`.
pub struct ExtendedStatisticsIterator<'a>
{
	pub(crate) extended_statistic_names: &'a [&'static str],
	pub(crate) extended_statistic_entries: Vec<rte_eth_xstat>,
	pub(crate) index: usize,
}

impl<'a> Iterator for ExtendedStatisticsIterator<'a>
{
	type Item = (&'static str, u64);
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.index == self.extended_statistic_names.len()
		{
			None
		}
		else
		{
			let index = self.index;
			let entry = unsafe { self.extended_statistic_entries.get_unchecked(index) };
			let name = *self.extended_statistic_names.get(entry.id as usize).unwrap();
			let statistic = entry.value;
			
			let result = Some((name, statistic));
			
			self.index = index + 1;
			
			result
		}
	}
}

impl<'a> ExtendedStatisticsIterator<'a>
{
	#[inline(always)]
	pub(crate) fn new_unchecked(extended_statistic_names: &'a [&'static str]) -> Self
	{
		Self
		{
			extended_statistic_names,
			extended_statistic_entries:
			{
				let mut extended_statistic_entries = Vec::with_capacity(extended_statistic_names.len());
				unsafe { extended_statistic_entries.set_len(extended_statistic_names.len()) };
				extended_statistic_entries
			},
			index: 0,
		}
	}
}
