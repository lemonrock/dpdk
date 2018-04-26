// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait to unify ArrayVec and Vec.
pub trait UnifiedArrayVecAndVec<T>
{
	/// Obtain maximum capacity.
	#[inline(always)]
	fn maximum_capacity(&self) -> usize;
	
	/// Obtain length.
	#[inline(always)]
	fn length(&self) -> usize;
	
	/// Should return a pointer to just after the last element.
	#[inline(always)]
	fn mutable_pointer_at_length(&mut self, length: usize) -> *mut T;
	
	/// Set length.
	#[inline(always)]
	fn set_length(&mut self, length: usize);
	
	/// Truncates without dropping any members (sets length to zero).
	#[inline(always)]
	fn truncate_without_drop(&mut self)
	{
		self.set_length(0)
	}
}

impl<T> UnifiedArrayVecAndVec<T> for Vec<T>
{
	#[inline(always)]
	fn maximum_capacity(&self) -> usize
	{
		self.capacity()
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.len()
	}
	
	#[inline(always)]
	fn mutable_pointer_at_length(&mut self, length: usize) -> *mut T
	{
		unsafe { self.get_unchecked_mut(length) as *mut T }
	}
	
	#[inline(always)]
	fn set_length(&mut self, length: usize)
	{
		unsafe { self.set_len(length) };
	}
}

macro_rules! implement_for_array_vec
{
	($len: expr) =>
	{
		impl<T> UnifiedArrayVecAndVec<T> for ArrayVec<[T; $len]>
		{
			#[inline(always)]
			fn maximum_capacity(&self) -> usize
			{
				$len
			}
			
			#[inline(always)]
			fn length(&self) -> usize
			{
				self.len()
			}
			
			#[inline(always)]
			fn mutable_pointer_at_length(&mut self, length: usize) -> *mut T
			{
				unsafe { self.get_unchecked_mut(length) as *mut T }
			}
			
			#[inline(always)]
			fn set_length(&mut self, length: usize)
			{
				unsafe { self.set_len(length) };
			}
		}
	}
}
implement_for_array_vec!(0);
implement_for_array_vec!(1);
implement_for_array_vec!(2);
implement_for_array_vec!(3);
implement_for_array_vec!(4);
implement_for_array_vec!(5);
implement_for_array_vec!(6);
implement_for_array_vec!(7);
implement_for_array_vec!(8);
implement_for_array_vec!(9);
implement_for_array_vec!(10);
implement_for_array_vec!(11);
implement_for_array_vec!(12);
implement_for_array_vec!(13);
implement_for_array_vec!(14);
implement_for_array_vec!(15);
implement_for_array_vec!(16);
implement_for_array_vec!(17);
implement_for_array_vec!(18);
implement_for_array_vec!(19);
implement_for_array_vec!(20);
implement_for_array_vec!(21);
implement_for_array_vec!(22);
implement_for_array_vec!(23);
implement_for_array_vec!(24);
implement_for_array_vec!(25);
implement_for_array_vec!(26);
implement_for_array_vec!(27);
implement_for_array_vec!(28);
implement_for_array_vec!(29);
implement_for_array_vec!(30);
implement_for_array_vec!(31);
implement_for_array_vec!(32);
implement_for_array_vec!(40);
implement_for_array_vec!(48);
implement_for_array_vec!(50);
implement_for_array_vec!(56);
implement_for_array_vec!(64);
implement_for_array_vec!(72);
implement_for_array_vec!(96);
implement_for_array_vec!(100);
implement_for_array_vec!(128);
implement_for_array_vec!(160);
implement_for_array_vec!(192);
implement_for_array_vec!(200);
implement_for_array_vec!(224);
