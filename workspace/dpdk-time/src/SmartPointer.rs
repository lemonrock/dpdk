// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait abstracting Box, Rc, Arc and newtype wrappers of them for use with FFI.
pub trait SmartPointer: Deref
{
	/// Converts to a `NonNull<Self::Target>`.
	#[inline(always)]
	fn to_non_null(self) -> NonNull<Self::Target>;
	
	/// Converts from a `NonNull<Self::Target>` produced by `from_raw_mut()`.
	#[inline(always)]
	fn from_non_null(raw: NonNull<Self::Target>) -> Self;
}

impl<T> SmartPointer for Box<T>
{
	#[inline(always)]
	fn to_non_null(self) -> NonNull<Self::Target>
	{
		unsafe { NonNull::new_unchecked(Box::into_raw(self)) }
	}
	
	#[inline(always)]
	fn from_non_null(raw: NonNull<Self::Target>) -> Self
	{
		unsafe { Box::from_raw(raw.as_ptr()) }
	}
}

impl<T> SmartPointer for Rc<T>
{
	#[inline(always)]
	fn to_non_null(self) -> NonNull<Self::Target>
	{
		unsafe { NonNull::new_unchecked(Rc::into_raw(self) as *mut _) }
	}
	
	#[inline(always)]
	fn from_non_null(raw: NonNull<Self::Target>) -> Self
	{
		unsafe { Rc::from_raw(raw.as_ptr() as *const _) }
	}
}

impl<T> SmartPointer for Arc<T>
{
	#[inline(always)]
	fn to_non_null(self) -> NonNull<Self::Target>
	{
		unsafe { NonNull::new_unchecked(Arc::into_raw(self) as *mut _) }
	}
	
	#[inline(always)]
	fn from_non_null(raw: NonNull<Self::Target>) -> Self
	{
		unsafe { Arc::from_raw(raw.as_ptr() as *const _) }
	}
}
