// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Nanoseconds.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Nanoseconds(u64);

impl From<u8> for Nanoseconds
{
	#[inline(always)]
	fn from(nanoseconds: u8) -> Self
	{
		Nanoseconds(nanoseconds as u64)
	}
}

impl From<u16> for Nanoseconds
{
	#[inline(always)]
	fn from(nanoseconds: u16) -> Self
	{
		Nanoseconds(nanoseconds as u64)
	}
}

impl From<u32> for Nanoseconds
{
	#[inline(always)]
	fn from(nanoseconds: u32) -> Self
	{
		Nanoseconds(nanoseconds as u64)
	}
}

impl From<usize> for Nanoseconds
{
	#[inline(always)]
	fn from(nanoseconds: usize) -> Self
	{
		Nanoseconds(nanoseconds as u64)
	}
}

impl From<u64> for Nanoseconds
{
	#[inline(always)]
	fn from(nanoseconds: u64) -> Self
	{
		Nanoseconds(nanoseconds)
	}
}

impl<'a> From<&'a timespec> for Nanoseconds
{
	#[inline(always)]
	fn from(nanoseconds: &'a timespec) -> Self
	{
		Nanoseconds(unsafe { rust_rte_timespec_to_ns(nanoseconds) })
	}
}

impl Into<u64> for Nanoseconds
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Into<timespec> for Nanoseconds
{
	#[inline(always)]
	fn into(self) -> timespec
	{
		unsafe { rust_rte_ns_to_timespec(self.0) }
	}
}
