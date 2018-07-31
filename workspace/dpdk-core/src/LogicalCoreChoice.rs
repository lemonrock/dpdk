// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A choice of logical core (CPU hyper thread) to use.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
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
			Any => Self::LCORE_ID_ANY,
			
			Specific(LogicalCore(value)) => value as u32,
		}
	}
}

impl LogicalCoreChoice
{
	/// DPDK constant.
	const LCORE_ID_ANY: u32 = ::std::u32::MAX;
	
	/// For current CPU.
	///
	/// Returns `Any` if this is not a DPDK EAL thread.
	///
	/// From a DPDK thread-local static.
	#[inline(always)]
	pub fn current_logical_core() -> Self
	{
		Self::from_u32(unsafe { per_lcore__lcore_id })
	}
	
	/// Unwraps as a LogicalCore.
	///
	/// Panics if not a LogicalCore.
	#[inline(always)]
	pub fn unwrap(self) -> LogicalCore
	{
		self.expect("This is not a logical core")
	}
	
	/// Unwraps as a LogicalCore.
	///
	/// Panics if not a LogicalCore.
	///
	/// Takes a `message` for the panic.
	#[inline(always)]
	pub fn expect(self, message: &str) -> LogicalCore
	{
		use self::LogicalCoreChoice::*;
		
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
			
			Specific(logical_core) => logical_core,
		}
	}
	
	//noinspection SpellCheckingInspection
	/// Constructs from an `u32` value.
	///
	/// Panics if the value is out-of-range.
	#[inline(always)]
	pub fn from_u32(value: u32) -> Self
	{
		use self::LogicalCoreChoice::*;
		
		if unlikely!(value == Self::LCORE_ID_ANY)
		{
			Any
		}
		else
		{
			debug_assert!((value as u32) < (::std::u16::MAX as u32), "value '{}' exceeds ::std::u16::MAX '{}'", value, ::std::u16::MAX);
			Specific(LogicalCore::from_u16(value as u16).unwrap())
		}
	}
}
