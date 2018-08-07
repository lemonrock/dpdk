// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a hash table optimized for hardware transactional memory and the AVX2 and SSE instruction sets, which instead of values stores indices.
///
/// The indices are allocated and freed internally by the hash table; the range of indices is from `0 .. capacity` (exclusive).
///
/// `capacity` is limited to 1,073,741,824 (`RTE_HASH_ENTRIES_MAX`).
///
/// A typical pattern of usage for this hash table is to pair it with an array or Vector of memory of the same capacity.
///
/// Insertions may be thread unsafe or thread safe, depending on how the hash table was constructed.
/// The hash table does not resize in use.
/// The hash table is NUMA-aware, and is best used within one NUMA node.
/// The hash table provides in-place access to values.
///
/// A limitation in the DPDK API means it is not possible to use Rust's `BuildHasher` trait, however, it can use the same default as Rust's hash map if `HasherType` is `std::collections::hash_map::DefaultHasher`.
///
/// Key equality can be either byte-wise or using the implementation of the trait `Eq`. In the latter case, insertion is thread unsafe.
#[derive(Debug)]
pub struct KeyToIndexHashTable<Key: Copy + Sized + Hash, HasherType: Hasher + Default>(HashTableInner<Key, HasherType>);

impl<Key: Copy + Sized + Hash + PartialEq, HasherType: Hasher + Default> KeyToIndexHashTable<Key, HasherType>
{
	/// Create a new instance, using Key's implementation of the `PartialEq` trait.
	///
	/// * `name` must be 32 characters or less.
	/// * `capacity` can not exceed 1,073,741,824 (`RTE_HASH_ENTRIES_MAX`).
	/// * `enable_hardware_transactional_memory_support` will only work on Hashwell, Skylake and later CPUs with suitable firmware fixes; it may be implemented in the future on ARM and PowerPC CPUs.
	/// * `enable_thread_safe_insert` always insertion to be thread safe. If `enable_hardware_transactional_memory_support` is specified then thread safe insertion is optimized with hardware transactional memory usage.
	#[inline(always)]
	pub fn new(name: &CStr, capacity: usize, allocate_on: NumaNode, enable_hardware_transactional_memory_support: bool) -> Result<Self, ()>
	{
		HashTableInner::new(name, capacity, allocate_on, enable_hardware_transactional_memory_support).map(|inner| KeyToIndexHashTable(inner))
	}
}

impl<Key: Copy + Sized + Hash, HasherType: Hasher + Default> KeyToIndexHashTable<Key, HasherType>
{
	/// Create a new instance, with Key equality being byte-wise.
	///
	/// * `name` must be 32 characters or less.
	/// * `capacity` can not exceed 1,073,741,824 (`RTE_HASH_ENTRIES_MAX`).
	/// * `enable_hardware_transactional_memory_support` will only work on Hashwell, Skylake and later CPUs with suitable firmware fixes; it may be implemented in the future on ARM and PowerPC CPUs.
	/// * `enable_thread_safe_insert` always insertion to be thread safe. If `enable_hardware_transactional_memory_support` is specified then thread safe insertion is optimized with hardware transactional memory usage.
	#[inline(always)]
	pub fn new_with_byte_wise_key_equality(name: &CStr, capacity: usize, allocate_on: NumaNode, enable_hardware_transactional_memory_support: bool, enable_thread_safe_insert: bool) -> Result<Self, ()>
	{
		HashTableInner::new_with_byte_wise_key_equality(name, capacity, allocate_on, enable_hardware_transactional_memory_support, enable_thread_safe_insert).map(|inner| KeyToIndexHashTable(inner))
	}
	
	/// Find an existing instance.
	///
	/// * `name` must be 32 characters or less.
	#[inline(always)]
	pub fn find_existing(name: &CStr) -> Option<Self>
	{
		HashTableInner::find_existing(name).map(|inner| KeyToIndexHashTable(inner))
	}
	
	/// Clears (resets) this this instance with initial values.
	#[inline(always)]
	pub fn clear(&self)
	{
		self.0.clear()
	}
	
	/// Inserts a key-value entry.
	///
	/// Not obvious what underlying DPDK implementation does with duplicate keys.
	///
	/// Returns an error if out of space (ie over capacity).
	///
	/// Only multi-thread safe if constructed with `new_with_byte_wise_key_equality` and `enable_thread_safe_insert` was `true`.
	#[inline(always)]
	pub fn insert(&self, key: Key) -> Result<usize, ()>
	{
		let result = unsafe { rte_hash_add_key(self.handle(), &key as *const Key as *const _) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else
		{
			match result
			{
				NegativeE::ENOSPC => Err(()),
				NegativeE::EINVAL => panic!("Parameters are invalid for rte_hash_add_key_data"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_add_key_data", unknown)
			}
		}
	}
	
	/// Inserts a key-value entry, using a precomputed hash (might be occasionally more optimal).
	///
	/// Not obvious what underlying DPDK implementation does with duplicate keys.
	///
	/// Returns an error if out of space (ie over capacity).
	///
	/// Only multi-thread safe if constructed with `new_with_byte_wise_key_equality` and `enable_thread_safe_insert` was `true`.
	#[inline(always)]
	pub fn insert_with_precomputed_key_hash(&self, key: Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Result<usize, ()>
	{
		let result = unsafe { rte_hash_add_key_with_hash(self.handle(), &key as *const Key as *const _, precomputed_key_hash.0) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else
		{
			match result
			{
				NegativeE::ENOSPC => Err(()),
				NegativeE::EINVAL => panic!("Parameters are invalid for rte_hash_add_key_data"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_add_key_data", unknown)
			}
		}
	}
	
	/// Removes an entry.
	///
	/// Not thread safe, even if `Self::new_with_byte_wise_key_equality` was called with `enable_thread_safe_insert` set to `true`.
	///
	/// Returns `None` if the key was not present.
	///
	/// Returns `Some(index)` if the key was present.
	#[inline(always)]
	pub fn remove(&self, key: &Key) -> Option<usize>
	{
		self.0.remove(key)
	}
	
	/// Removes an entry, using a precomputed hash (might be occasionally more optimal).
	///
	/// Not thread safe, even if `Self::new_with_byte_wise_key_equality` was called with `enable_thread_safe_insert` set to `true`.
	///
	/// Returns `None` if the key was not present.
	///
	/// Returns `Some(index)` if the key was present.
	#[inline(always)]
	pub fn remove_with_precomputed_key_hash(&self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Option<usize>
	{
		self.0.remove_with_precomputed_key_hash(key, precomputed_key_hash)
	}
	
	/// Finds an entry.
	///
	/// Always thread safe.
	///
	/// Returns `None` if the key was not present.
	///
	/// Returns `Some(index)` if the key was present.
	#[inline(always)]
	pub fn look_up(&self, key: &Key) -> Option<usize>
	{
		let result = unsafe { rte_hash_lookup(self.handle(), key as *const Key as *const _) };
		if likely!(result >= 0)
		{
			Some(result as usize)
		}
		else
		{
			match result
			{
				NegativeE::ENOENT => None,
				NegativeE::EINVAL => panic!("Parameters are invalid for rte_hash_lookup_data"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_lookup_data", unknown)
			}
		}
	}
	
	/// Finds an entry, using a precomputed hash (might be occasionally more optimal).
	///
	/// Always thread safe.
	///
	/// Returns `None` if the key was not present.
	///
	/// Returns `Some(index)` if the key was present.
	#[inline(always)]
	pub fn look_up_with_precomputed_key_hash(&self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Option<usize>
	{
		let result = unsafe { rte_hash_lookup_with_hash(self.handle(), key as *const Key as *const _, precomputed_key_hash.0) };
		if likely!(result >= 0)
		{
			Some(result as usize)
		}
		else
		{
			match result
			{
				NegativeE::ENOENT => None,
				NegativeE::EINVAL => panic!("Parameters are invalid for rte_hash_lookup_data"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_lookup_data", unknown)
			}
		}
	}
	
	/// Finds entries.
	///
	/// Always thread safe.
	#[inline(always)]
	pub fn look_up_bulk(&self, keys: &ArrayVec<[&Key; LookUpBulkMaximum]>, mut result_handler: impl LookUpBulkResultHandler<Key, usize>)
	{
		let mut positions: ArrayVec<[i32; LookUpBulkMaximum]> = ArrayVec::new();
		
		let length = keys.len();
		let result = unsafe { rte_hash_lookup_bulk(self.handle(), keys.as_ptr() as *const &Key as *const *const Key as *const *const _ as *mut *const _, length as u32, positions.as_mut_ptr()) };
		if likely!(result >= 0)
		{
			unsafe { positions.set_len(length) };
			
			let mut index = 0;
			while index < length
			{
				let position = unsafe { * positions.get_unchecked(index) };
				if likely!(position >= 0)
				{
					result_handler.key_found(keys, index, position as usize)
				}
				else
				{
					match position
					{
						NegativeE::ENOENT => result_handler.key_not_present(keys, index),
						unknown @ _ => panic!("Unknown error '{}' at index '{}' from rte_hash_lookup_bulk", unknown, index)
					}
				}
				
				index += 1;
			}
		}
		else
		{
			match result
			{
				NegativeE::EINVAL => panic!("Parameters are invalid for rte_hash_lookup_bulk"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_lookup_bulk", unknown)
			}
		}
	}
	
	/// Iterate over key-value pairs.
	#[inline(always)]
	pub fn iterate<'a>(&'a self) -> KeyToIndexHashTableIterator<'a, Key, HasherType>
	{
		KeyToIndexHashTableIterator::new(self)
	}
	
	/// Given an index, find the associated key.
	#[inline(always)]
	pub fn index_to_key(&self, index: usize) -> Option<&Key>
	{
		debug_assert!(index <= ::std::i32::MAX as usize, "index '{}' is larger than ::std::i32::MAX '{}'", index, ::std::i32::MAX);
		
		let mut key: &Key = unsafe { uninitialized() };
		let result = unsafe { rte_hash_get_key_with_position(self.handle(), index as i32, &mut key as *mut &Key as *mut *const Key as *mut *mut Key as *mut *mut _) };
		if likely!(result == 0)
		{
			Some(key)
		}
		else
		{
			match result
			{
				E::ENOENT => None,
				E::EINVAL => panic!("Parameters are invalid for rte_hash_get_key_with_position"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_get_key_with_position", unknown)
			}
		}
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_hash
	{
		self.0.handle()
	}
}
