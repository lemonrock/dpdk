// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Iterator over all slave logical cores (ie are logical cores excluding master).
///
/// Create using `SlaveLogicalCoreIterator::default()`.
pub struct SlaveLogicalCoreIterator
{
	master: LogicalCore,
	all_logical_core_iterator: AllLogicalCoreIterator,
}

impl Default for SlaveLogicalCoreIterator
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			master: LogicalCore::master(),
			all_logical_core_iterator: AllLogicalCoreIterator::default(),
		}
	}
}

impl Iterator for SlaveLogicalCoreIterator
{
	type Item = LogicalCore;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		loop
		{
			match self.all_logical_core_iterator.next()
			{
				None => return None,
				Some(logical_core) => if likely!(logical_core != self.master)
				{
					return Some(logical_core)
				}
				else
				{
					continue
				},
			}
		}
	}
}
