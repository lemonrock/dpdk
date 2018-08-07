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
pub struct ArrayIndexHashTable<Key: Copy + Sized + Hash, HasherType: Hasher + Default>(HashTableInner<Key, HasherType>);

impl<Key: Copy + Sized + Hash + PartialEq, HasherType: Hasher + Default> ArrayIndexHashTable<Key, HasherType>
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
		HashTableInner::new(name, capacity, allocate_on, enable_hardware_transactional_memory_support).map(|inner| ArrayIndexHashTable(inner))
	}
}

impl<Key: Copy + Sized + Hash, HasherType: Hasher + Default> ArrayIndexHashTable<Key, HasherType>
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
		HashTableInner::new_with_byte_wise_key_equality(name, capacity, allocate_on, enable_hardware_transactional_memory_support, enable_thread_safe_insert).map(|inner| ArrayIndexHashTable(inner))
	}
	
	/// Find an existing instance.
	///
	/// * `name` must be 32 characters or less.
	#[inline(always)]
	pub fn find_existing(name: &CStr) -> Option<Self>
	{
		HashTableInner::find_existing(name).map(|inner| ArrayIndexHashTable(inner))
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
				E::ENOSPC => Err(()),
				E::EINVAL => panic!("Parameters are invalid for rte_hash_add_key_data"),
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
				E::ENOSPC => Err(()),
				E::EINVAL => panic!("Parameters are invalid for rte_hash_add_key_data"),
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
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_hash
	{
		self.0.handle()
	}
}
