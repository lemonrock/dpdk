// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents an iterator over extended statistics.
///
/// Obtain using `EthernetDeviceCapabilities`.
pub struct ExtendedStatisticsIterator<'a>
{
	pub(crate) extended_statistic_names: &'a [&'static str],
	pub(crate) extended_statistic_statistics: Vec<u64>,
	pub(crate) identifier: usize,
}

impl<'a> Iterator for ExtendedStatisticsIterator<'a>
{
	type Item = (&'static str, u64);
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.identifier == self.extended_statistic_names.len()
		{
			None
		}
		else
		{
			let identifier = self.identifier;
			let name = * unsafe { self.extended_statistic_names.get_unchecked(identifier) };
			let statistic = * unsafe { self.extended_statistic_statistics.get_unchecked(identifier) };
			
			let result = Some((name, statistic));
			
			self.identifier = identifier + 1;
			
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
			extended_statistic_statistics:
			{
				let size = extended_statistic_names.len();
				
				let mut extended_statistic_statistics = Vec::with_capacity(size);
				
				for _ in 0 .. size
				{
					extended_statistic_statistics.push(0)
				}
				
				extended_statistic_statistics
			},
			identifier: 0,
		}
	}
	
	#[inline(always)]
	pub(crate) fn reset(&mut self)
	{
		self.identifier = 0
	}
	
	#[inline(always)]
	pub(crate) fn size(&self) -> usize
	{
		self.extended_statistic_statistics.len()
	}
	
	#[inline(always)]
	pub(crate) fn values_pointer(&mut self) -> *mut u64
	{
		self.extended_statistic_statistics.as_mut_ptr()
	}
}
