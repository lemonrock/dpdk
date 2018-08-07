// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An iterator over a UsizeHashTable.
#[derive(Debug)]
pub struct UsizeHashTableIterator<'a, Key: 'a + Copy + Sized + Hash, Value: 'a + UsizeHashTableValue, HasherType: 'a + Hasher + Default>
{
	table: &'a UsizeHashTable<Key, Value, HasherType>,
	next: u32,
}

impl<'a, Key: 'a + Copy + Sized + Hash, Value: 'a + UsizeHashTableValue, HasherType: 'a + Hasher + Default> Iterator for UsizeHashTableIterator<'a, Key, Value, HasherType>
{
	type Item = (&'a Key, Value);
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let mut key: &Key = unsafe { uninitialized() };
		let mut data: usize = unsafe { uninitialized() };
		
		let result = unsafe { rte_hash_iterate(self.table.handle(), &mut key as *mut &Key as *mut *const Key as *mut *const _, &mut data as *mut usize as *mut *mut _, &mut self.next) };
		
		if likely!(result >= 0)
		{
			Some((key, Value::from_usize(data)))
		}
		else
		{
			match result
			{
				NegativeE::ENOENT => None,
				NegativeE::EINVAL => panic!("Parameters are invalid for rte_hash_iterate"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_iterate", unknown)
			}
		}
	}
}

impl<'a, Key: Copy + Sized + Hash, Value: UsizeHashTableValue, HasherType: Hasher + Default> UsizeHashTableIterator<'a, Key, Value, HasherType>
{
	#[inline(always)]
	pub(crate) fn new(table: &'a UsizeHashTable<Key, Value, HasherType>) -> Self
	{
		Self
		{
			table,
			next: 0,
		}
	}
}
