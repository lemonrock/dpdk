// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Pointer extensions.
pub trait PointerExt: Sized
{
	/// To Memory.
	#[inline(always)]
	fn offset_16(self) -> Memory
	{
		self._offset(16)
	}
	
	/// To Memory.
	#[inline(always)]
	fn offset_32(self) -> Memory
	{
		self._offset(32)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _offset(self, increment: usize) -> Memory
	{
		unsafe { NonNull::new_unchecked(self.to_memory().as_ptr().offset(increment as isize)) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn to_memory(self) -> Memory;
	
	/// Is not null.
	#[inline(always)]
	fn is_not_null(self) -> bool;
}

impl PointerExt for *const u8
{
	#[inline(always)]
	fn to_memory(self) -> Memory
	{
		debug_assert!(self.is_not_null(), "is null");
		
		unsafe { NonNull::new_unchecked(self as *mut _) }
	}
	
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		!self.is_null()
	}
}

impl PointerExt for *mut u8
{
	#[inline(always)]
	fn to_memory(self) -> Memory
	{
		debug_assert!(self.is_not_null(), "is null");
		
		unsafe { NonNull::new_unchecked(self) }
	}
	
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		!self.is_null()
	}
}

impl PointerExt for *mut c_void
{
	#[inline(always)]
	fn to_memory(self) -> Memory
	{
		debug_assert!(self.is_not_null(), "is null");
		
		unsafe { NonNull::new_unchecked(self as *mut u8) }
	}
	
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		!self.is_null()
	}
}

impl PointerExt for *mut i8
{
	#[inline(always)]
	fn to_memory(self) -> Memory
	{
		debug_assert!(self.is_not_null(), "is null");
		
		unsafe { NonNull::new_unchecked(self as *mut u8) }
	}
	
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		!self.is_null()
	}
}

impl PointerExt for *const i8
{
	#[inline(always)]
	fn to_memory(self) -> Memory
	{
		debug_assert!(self.is_not_null(), "is null");
		
		unsafe { NonNull::new_unchecked(self as *mut u8) }
	}
	
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		!self.is_null()
	}
}
