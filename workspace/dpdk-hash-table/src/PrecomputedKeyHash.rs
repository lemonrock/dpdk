// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A precomputed hash for a key.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PrecomputedKeyHash<Key: Copy + Sized + Hash, HasherType: Hasher + Default>(u32, PhantomData<(Key, HasherType)>);

impl<Key: Copy + Sized + Hash, HasherType: Hasher + Default> PrecomputedKeyHash<Key, HasherType>
{
	/// Precomputes the key's hash.
	#[inline(always)]
	pub fn precompute(key: &Key) -> Self
	{
		PrecomputedKeyHash(Self::compute_hash(key), PhantomData)
	}
	
	#[inline(always)]
	pub(crate) fn compute_hash(key: &Key) -> u32
	{
		let mut hasher = HasherType::default();
		key.hash(&mut hasher);
		let result64 = hasher.finish();
		
		unsafe
		{
			let pointer = &result64 as *const u64 as *const u32;
			let lower = *pointer;
			let upper = *pointer.offset(1);
			lower ^ upper
		}
	}
	
}
