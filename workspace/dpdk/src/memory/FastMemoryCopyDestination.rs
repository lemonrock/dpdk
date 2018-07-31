// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Fast memory copy routines from DPDK.
pub trait FastMemoryCopyDestination
{
	#[doc(hidden)]
	#[inline(always)]
	fn as_usize(self) -> usize;
	
	#[doc(hidden)]
	#[inline(always)]
	fn as_mutable_pointer(self) -> *mut u8;
	
	#[doc(hidden)]
	#[inline(always)]
	fn increment(self, increment: usize) -> Self;
}

impl FastMemoryCopyDestination for *mut u8
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		self as usize
	}
	
	#[inline(always)]
	fn as_mutable_pointer(self) -> *mut u8
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		self
	}
	
	#[inline(always)]
	fn increment(self, increment: usize) -> Self
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		unsafe { self.offset(increment as isize) }
	}
}

impl FastMemoryCopyDestination for NonNull<u8>
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.as_ptr() as usize
	}
	
	#[inline(always)]
	fn as_mutable_pointer(self) -> *mut u8
	{
		self.as_ptr()
	}
	
	#[inline(always)]
	fn increment(self, increment: usize) -> Self
	{
		unsafe { NonNull::new_unchecked(self.as_ptr().offset(increment as isize)) }
	}
}

impl FastMemoryCopyDestination for usize
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		debug_assert_ne!(self, 0, "null");
		
		self
	}
	
	#[inline(always)]
	fn as_mutable_pointer(self) -> *mut u8
	{
		debug_assert_ne!(self, 0, "null");
		
		self as *mut u8
	}
	
	#[inline(always)]
	fn increment(self, increment: usize) -> Self
	{
		debug_assert_ne!(self, 0, "null");
		
		self + increment
	}
}
