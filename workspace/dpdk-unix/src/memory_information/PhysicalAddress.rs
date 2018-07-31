// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Physical Address.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PhysicalAddress(u64);

impl Into<u64> for PhysicalAddress
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Add<u64> for PhysicalAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u64) -> Self::Output
	{
		PhysicalAddress(self.0 + rhs)
	}
}

impl AddAssign<u64> for PhysicalAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u64)
	{
		self.0 += rhs
	}
}

impl Add<usize> for PhysicalAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: usize) -> Self::Output
	{
		PhysicalAddress(self.0 + (rhs as u64))
	}
}

impl AddAssign<usize> for PhysicalAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: usize)
	{
		self.0 += rhs as u64
	}
}

impl PhysicalAddress
{
	/// Relative offset from the start of the system page containing this physical address.
	///
	/// May be zero, but will always be less than the system page size.
	#[inline(always)]
	pub fn offset_from_start_of_page(self) -> u64
	{
		self.0 % (page_size() as u64)
	}
	
	/// The address of the page which contains this physical address.
	/// May be the same value.
	#[inline(always)]
	pub fn first_address_in_page(self) -> Self
	{
		PhysicalAddress(self.0 & !((page_size() as u64) - 1))
	}
}
