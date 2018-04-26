// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A choice of NUMA node to use.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialization)]
pub enum NumaNodeChoice
{
	/// Equivalent to DPDK's `SOCKET_ID_ANY`.
	Any,
	
	/// A specific node.
	Specific(NumaNode),
}

impl Into<i32> for NumaNodeChoice
{
	#[inline(always)]
	fn into(self) -> i32
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Any => SOCKET_ID_ANY,
			
			Specific(NumaNode(value)) => value as i32,
		}
	}
}

impl Into<u32> for NumaNodeChoice
{
	/// Note that Any (`SOCKET_ID_ANY`) is represented as `::std::u32::MAX`.
	///
	/// This functionality primarily exists because some DPDK APIs (eg `rte_reorder_create`) take an unsigned int when they should take a signed int...
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Any => ::std::u32::MAX,
			
			Specific(NumaNode(value)) => value as u32,
		}
	}
}

impl NumaNodeChoice
{
	/// For current CPU.
	///
	/// Slightly slow as must go via a C function call.
	#[inline(always)]
	pub fn for_current_cpu() -> Self
	{
		Self::from_i32(unsafe { rte_socket_id() })
	}
	
	#[inline(always)]
	pub fn unwrap(self) -> NumaNode
	{
		self.expect("This is not a logical core")
	}
	
	#[inline(always)]
	pub fn expect(self, message: &str) -> NumaNode
	{
		use self::NumaNodeChoice::*;
		
		// This is a separate function similar to that used by ::std::option::Option.
		#[inline(never)]
		#[cold]
		fn expect_failed(message: &str) -> !
		{
			panic!("{}", message)
		}
		
		match self
		{
			Any => expect_failed(message),
			
			Specific(numa_node) => numa_node,
		}
	}
	
	/// Constructs from an `i32` value.
	///
	/// Panics if the value is out-of-range (less than -1 or greater than or equal to `RTE_MAX_NUMA_NODES`).
	#[inline(always)]
	pub fn from_i32(value: i32) -> Self
	{
		use self::NumaNodeChoice::*;
		
		if likely(value >= 0)
		{
			debug_assert!((RTE_MAX_NUMA_NODES as u16) <= (::std::u8::MAX as u16), "RTE_MAX_NUMA_NODES '{}' exceeds ::std::u8::MAX; the DPDK API is broken", RTE_MAX_NUMA_NODES, ::std::u8::MAX);
			
			assert!((value as u32) < (RTE_MAX_NUMA_NODES as u32), "value '{}' equals or exceeds RTE_MAX_NUMA_NODES '{}'", value, RTE_MAX_NUMA_NODES);
			
			Specific(NumaNode(value))
		}
		else if unlikely(value == SOCKET_ID_ANY)
		{
			Any
		}
		else
		{
			panic!("value '{}' is invalid for a NUMA node", value)
		}
	}
	
	/// Converts to an `i32` value.
	#[inline(always)]
	pub fn to_i32(self) -> i32
	{
	
	}
}
