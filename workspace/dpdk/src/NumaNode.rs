// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// NUMA node numbers start at zero.
///
/// It is assumed by DPDK code that there is always at least one NUMA node, and, if there is one NUMA node, it is number zero.
///
/// Some DPDK APIs (eg `rte_eth_dev_socket_id`) treat zero as also meaning 'undetermined'.
///
/// NUMA node numbers are not necessarily contiguous but usually are.
///
/// NUMA nodes are also, confusingly, known as sockets. In this sense they represent the socket where a modern CPU with multiple cores resides.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialization)]
pub struct NumaNode(u8);

impl Into<u8> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Into<u16> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
	}
}

impl Into<u32> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Into<u64> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<usize> for NumaNode
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl NumaNode
{
	/// Maximum number of NUMA sockets.
	pub const MaximumNumaSockets: usize = RTE_MAX_NUMA_NODES;
	
	/// Constructs from an `u32` value.
	///
	/// Panics if the value is out-of-range greater than or equal to `RTE_MAX_NUMA_NODES`).
	#[inline(always)]
	pub fn from_u32(value: u32) -> Self
	{
		debug_assert!((RTE_MAX_NUMA_NODES as u32) <= (::std::u8::MAX as u32), "RTE_MAX_NUMA_NODES '{}' exceeds ::std::u8::MAX; the DPDK API is broken", RTE_MAX_NUMA_NODES, ::std::u8::MAX);
		
		assert!(value < (RTE_MAX_NUMA_NODES as u32), "value '{}' equals or exceeds RTE_MAX_NUMA_NODES '{}'", value, RTE_MAX_NUMA_NODES);
		
		NumaNode(value as u8)
	}
}
