// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



/// Logical core numbers start at zero.
///
/// Logical core numbers are not necessarily contiguous but usually are.
///
/// A logical core is equivalent to a hyper thread.
///
/// Not all logical cores may have been assigned to be used by DPDK, and, of those that have, they may have one of three roles:-
///
/// * Master
/// * Slave
/// * Service
///
/// Only one core of all cores in a process can be a Master.
///
/// A logical core belongs to a NUMA node.
///
/// DPDK 18.02 defaults to a maximum of 128 logical cores.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialization)]
pub struct LogicalCore(u16);

impl Into<u16> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
	}
}

impl Into<u32> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Into<u64> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<usize> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl LogicalCore
{
}
