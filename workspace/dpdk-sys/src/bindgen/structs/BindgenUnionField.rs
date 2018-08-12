// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(PartialOrd, Ord)]
pub struct BindgenUnionField<T>(PhantomData<T>);

impl<T> BindgenUnionField<T>
{
	
	#[inline(always)]
	pub fn new() -> Self
	{
		BindgenUnionField(PhantomData)
	}
	
	#[inline(always)]
	pub unsafe fn as_ref(&self) -> &T
	{
		transmute(self)
	}
	
	#[inline(always)]
	pub unsafe fn as_mut(&mut self) -> &mut T
	{
		transmute(self)
	}
}

impl<T> Default for BindgenUnionField<T>
{
	
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl<T> Clone for BindgenUnionField<T>
{
	
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::new()
	}
}

impl<T> Copy for BindgenUnionField<T>
{
}

impl<T> Debug for BindgenUnionField<T>
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> Result
	{
		fmt.write_str("BindgenUnionField")
	}
}

impl<T> Hash for BindgenUnionField<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, _state: &mut H)
	{
	}
}

impl<T> PartialEq for BindgenUnionField<T>
{
	#[inline(always)]
	fn eq(&self, _other: &BindgenUnionField<T>) -> bool
	{
		true
	}
}

impl<T> Eq for BindgenUnionField<T>
{
}
