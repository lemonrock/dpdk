// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Default)]
pub struct IncompleteArrayField<T>(PhantomData<T>);

impl<T> IncompleteArrayField<T>
{
	
	#[inline(always)]
	pub fn new() -> Self
	{
		IncompleteArrayField(PhantomData)
	}
	
	#[inline(always)]
	pub unsafe fn as_ptr(&self) -> *const T
	{
		transmute(self)
	}
	
	#[inline(always)]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut T
	{
		transmute(self)
	}
	
	#[inline(always)]
	pub unsafe fn as_slice(&self, len: usize) -> &[T]
	{
		from_raw_parts(self.as_ptr(), len)
	}
	
	#[inline(always)]
	pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T]
	{
		from_raw_parts_mut(self.as_mut_ptr(), len)
	}
}

impl<T> Debug for IncompleteArrayField<T>
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> Result
	{
		fmt.write_str("IncompleteArrayField")
	}
}

impl<T> Clone for IncompleteArrayField<T>
{
	
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::new()
	}
}

impl<T> Copy for IncompleteArrayField<T>
{
}
