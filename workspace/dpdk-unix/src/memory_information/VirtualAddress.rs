// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A virtual address.
///
/// This is the same as the value returned from `malloc()`, a `*mut T` pointer, a `&T` reference, etc.
///
/// No checks are made for its validity.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VirtualAddress(usize);

impl<T> From<NonNull<T>> for VirtualAddress
{
	#[inline(always)]
	fn from(value: NonNull<T>) -> Self
	{
		VirtualAddress(value.as_ptr() as usize)
	}
}

impl<T> From<Option<NonNull<T>>> for VirtualAddress
{
	#[inline(always)]
	fn from(value: Option<NonNull<T>>) -> Self
	{
		let address = match value
		{
			None => 0,
			Some(value) => value.as_ptr() as usize,
		};
		VirtualAddress(address)
	}
}

impl<T> Into<Option<NonNull<T>>> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> Option<NonNull<T>>
	{
		NonNull::new(self.0 as *mut T)
	}
}

impl<'a, T: 'a> From<&'a T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: &'a T) -> Self
	{
		VirtualAddress(value as *const T as usize)
	}
}

impl<'a, T: 'a> From<&'a mut T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: &'a mut T) -> Self
	{
		VirtualAddress(value as *mut T as usize)
	}
}

impl<T> From<*const T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: *const T) -> Self
	{
		VirtualAddress(value as usize)
	}
}

impl<T> Into<*const T> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> *const T
	{
		self.0 as *const T
	}
}

impl<T> From<*mut T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: *mut T) -> Self
	{
		VirtualAddress(value as usize)
	}
}

impl<T> Into<*mut T> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> *mut T
	{
		self.0 as *mut T
	}
}

impl From<usize> for VirtualAddress
{
	#[inline(always)]
	fn from(value: usize) -> Self
	{
		VirtualAddress(value)
	}
}

impl Into<usize> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0
	}
}

impl Add<usize> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: usize) -> Self::Output
	{
		VirtualAddress(self.0 + rhs)
	}
}

impl AddAssign<usize> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: usize)
	{
		self.0 += rhs
	}
}

impl Sub<usize> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: usize) -> Self::Output
	{
		VirtualAddress(self.0 - rhs)
	}
}

impl SubAssign<usize> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: usize)
	{
		self.0 -= rhs
	}
}

impl VirtualAddress
{
	/// Relative offset from the start of the system page containing this virtual address.
	///
	/// May be zero, but will always be less than the system page size.
	#[inline(always)]
	pub fn offset_from_start_of_page(self) -> usize
	{
		self.0 % page_size()
	}
	
	/// The address of the page which contains this physical address.
	/// May be the same value.
	#[inline(always)]
	pub fn first_address_in_page(self) -> Self
	{
		VirtualAddress(self.0 & !(page_size() - 1))
	}
	
	/// This function will translate virtual addresses to physical addresses.
	///
	/// Before using this function, the memory reference by a virtual address should have been `mlock()`'d.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn virtual_addresses_to_physical_addresses<HVA: HasVirtualAddress>(have_virtual_addresses: impl Iterator<Item=HVA>, mut physical_address_user: impl FnMut(HVA, Self, PhysicalAddress))
	{
		PageMapEntry::read_our_pagemap_file(have_virtual_addresses, |has_virtual_address, virtual_address, page_map_entry|
		{
			let physical_address_of_physical_page: PhysicalAddress = page_map_entry.physical_page_frame_number().into();
			let physical_address = physical_address_of_physical_page.add(virtual_address.offset_from_start_of_page());
			physical_address_user(has_virtual_address, virtual_address, physical_address)
		}).expect("could not read pagemap");
	}
}
