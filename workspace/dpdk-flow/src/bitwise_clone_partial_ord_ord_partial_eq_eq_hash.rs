// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


macro_rules! bitwise_clone_partial_ord_ord_partial_eq_eq_hash
{
	($type: tt) =>
	{
		impl Clone for $type
		{
			#[inline(always)]
			fn clone(&self) -> Self
			{
				let mut clone = unsafe { uninitialized() };
				unsafe { copy_nonoverlapping(self, &mut clone, size_of::<$type>()) };
				clone
			}
		}
		
		impl PartialOrd for $type
		{
			#[inline(always)]
			fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
			{
				Some(self.cmp(rhs))
			}
		}
		
		impl Ord for $type
		{
			#[inline(always)]
			fn cmp(&self, rhs: &Self) -> Ordering
			{
				self.mem_cmp(rhs)
			}
		}
		
		impl PartialEq for $type
		{
			#[inline(always)]
			fn eq(&self, rhs: &Self) -> bool
			{
				self.mem_eq(rhs)
			}
		}
		
		impl Eq for $type
		{
		}
		
		impl Hash for $type
		{
			#[inline(always)]
			fn hash<H: Hasher>(&self, hasher: &mut H)
			{
				let pointer = self as *const $type as *const u8;
				let size = size_of::<$type>();
				hasher.write(unsafe { from_raw_parts(pointer, size) })
			}
		}
	}
}
