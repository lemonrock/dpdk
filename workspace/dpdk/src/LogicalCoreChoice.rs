// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A choice of logical core (CPU hyper thread) to use.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialization)]
pub enum LogicalCoreChoice
{
	/// Equivalent to DPDK's `LCORE_ID_ANY`.
	Any,
	
	/// A specific logical core.
	Specific(LogicalCore),
}

impl Into<u32> for LogicalCoreChoice
{
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::LogicalCoreChoice::*;
		
		match self
		{
			Any => LCORE_ID_ANY,
			
			Specific(LogicalCore(value)) => value as u32,
		}
	}
}

impl LogicalCoreChoice
{
	/// For current CPU.
	#[inline(always)]
	pub fn for_current_cpu() -> Self
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Constructs from an `u32` value.
	///
	/// Panics if the value is out-of-range (greater than or equal to `RTE_MAX_LCORE`).
	#[inline(always)]
	pub fn from_u32(value: u32) -> Self
	{
		use self::LogicalCoreChoice::*;
		
		if unlikely(value == LCORE_ID_ANY)
		{
			Any
		}
		else
		{
			debug_assert!((RTE_MAX_LCORE as u64) <= (::std::u16::MAX as u16), "RTE_MAX_LCORE '{}' exceeds ::std::u16::MAX '{}'", RTE_MAX_LCORE, ::std::u16::MAX);
			
			if unlikely(value >= RTE_MAX_LCORE as u32)
			{
				panic!("value '{}' exceeds RTE_MAX_LCORE '{}'", value, RTE_MAX_LCORE)
			}
			
			Specific(LogicalCore(value as u16))
		}
	}
}
