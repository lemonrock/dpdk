// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Memory statistic unit.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryInformationUnit
{
	/// KiloByte.
	KiloByte,
	
	/// Numeric count.
	Count,
}

impl MemoryInformationUnit
{
	#[inline(always)]
	pub(crate) fn ends_with(&self) -> &'static str
	{
		match *self
		{
			MemoryInformationUnit::KiloByte => " kB",
			MemoryInformationUnit::Count => "",
		}
	}
}
