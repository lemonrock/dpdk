// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a hash table optimized for hardware transactional memory and the AVX2 and SSE instruction sets.
///
/// `capacity` is limited to 1,073,741,824 (`RTE_HASH_ENTRIES_MAX`).
///
/// Values may be no bigger than `usize`.
///
/// If storing `Box` or similar smart pointers as values in this hash table, then you'll need to `forget()` them when using `look_up()`, `look_up_with_precomputed_key_hash()` and `look_up_bulk()`, otherwise a double-free or use of previously freed memory is possible.
///
/// Values are ***not*** dropped when an instance of this struct is dropped.
///
/// Insertions may be thread unsafe or thread safe, depending on how the hash table was constructed.
/// The hash table does not resize in use.
/// The hash table is NUMA-aware, and is best used within one NUMA node.
/// The hash table provides in-place access to values.
///
/// A limitation in the DPDK API means it is not possible to use Rust's `BuildHasher` trait, however, it can use the same default as Rust's hash map if `HasherType` is `std::collections::hash_map::DefaultHasher`.
///
/// Key equality can be either byte-wise or using the implementation of the trait `Eq`. In the latter case, insertion is thread unsafe.
pub struct UsizeHashTable<Key: Copy + Sized + Hash, Value: UsizeHashTableValue, HasherType: Hasher + Default>(HashTableInner<Key, HasherType>, PhantomData<Value>);

impl<Key: Copy + Sized + Hash + PartialEq, Value: UsizeHashTableValue, HasherType: Hasher + Default> UsizeHashTable<Key, Value, HasherType>
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
		HashTableInner::new(name, capacity, allocate_on, enable_hardware_transactional_memory_support).map(|inner| UsizeHashTable(inner, PhantomData))
	}
}

impl<Key: Copy + Sized + Hash, Value: UsizeHashTableValue, HasherType: Hasher + Default> UsizeHashTable<Key, Value, HasherType>
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
		HashTableInner::new_with_byte_wise_key_equality(name, capacity, allocate_on, enable_hardware_transactional_memory_support, enable_thread_safe_insert).map(|inner| UsizeHashTable(inner, PhantomData))
	}
	
	/// Find an existing instance.
	///
	/// * `name` must be 32 characters or less.
	#[inline(always)]
	pub fn find_existing(name: &CStr) -> Option<Self>
	{
		HashTableInner::find_existing(name).map(|inner| UsizeHashTable(inner, PhantomData))
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
	pub fn insert(&self, key: Key, data: Value) -> Result<(), ()>
	{
		let result = unsafe { rte_hash_add_key_data(self.handle(), &key as *const Key as *const _, data.into_usize() as *mut _) };
		if likely!(result == 0)
		{
			Ok(())
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
	pub fn insert_with_precomputed_key_hash(&self, key: Key, data: Value, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Result<(), ()>
	{
		let result = unsafe { rte_hash_add_key_with_hash_data(self.handle(), &key as *const Key as *const _, precomputed_key_hash.0, data.into_usize() as *mut _) };
		if likely!(result == 0)
		{
			Ok(())
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
	/// Returns false if the key was not present.
	#[inline(always)]
	pub fn remove(&self, key: &Key) -> bool
	{
		self.0.remove(key).is_some()
	}
	
	/// Removes an entry, using a precomputed hash (might be occasionally more optimal).
	///
	/// Not thread safe, even if `Self::new_with_byte_wise_key_equality` was called with `enable_thread_safe_insert` set to `true`.
	///
	/// Returns false if the key was not present.
	#[inline(always)]
	pub fn remove_with_precomputed_key_hash(&self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> bool
	{
		self.0.remove_with_precomputed_key_hash(key, precomputed_key_hash).is_some()
	}
	
	/// Finds an entry.
	///
	/// Always thread safe.
	///
	/// Be very careful to use `forget()` if working with `Box` and similar smart pointers.
	#[inline(always)]
	pub fn look_up(&self, key: &Key) -> Option<Value>
	{
		let mut data: usize = unsafe { uninitialized() };
		
		let result = unsafe { rte_hash_lookup_data(self.handle(), key as *const Key as *const _, &mut data as *mut usize as *mut *mut _) };
		if likely!(result == 0)
		{
			Some(Value::from_usize(data))
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
	/// Be very careful to use `forget()` if working with `Box` and similar smart pointers.
	#[inline(always)]
	pub fn look_up_with_precomputed_key_hash(&self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Option<Value>
	{
		let mut data: usize = unsafe { uninitialized() };
		
		let result = unsafe { rte_hash_lookup_with_hash_data(self.handle(), key as *const Key as *const _, precomputed_key_hash.0, &mut data as *mut usize as *mut *mut _) };
		if likely!(result == 0)
		{
			Some(Value::from_usize(data))
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
	///
	/// Be very careful to use `forget()` if working with `Box` and similar smart pointers.
	#[inline(always)]
	pub fn look_up_bulk(&self, keys: &ArrayVec<[&Key; LookUpBulkMaximum]>, mut result_handler: impl LookUpBulkResultHandler<Key, Value>)
	{
		let mut hit_mask = unsafe { uninitialized() };
		let mut data: ArrayVec<[usize; LookUpBulkMaximum]> = ArrayVec::new();

		let length = keys.len();
	
		let result = unsafe { rte_hash_lookup_bulk_data(self.handle(), keys.as_ptr() as *const &Key as *const *const Key as *const *const _ as *mut *const _, length as u32, &mut hit_mask, data.as_mut_ptr() as *mut *mut _) };
		if likely!(result >= 0)
		{
			unsafe { data.set_len(length) };
			
			let mut index = 0;
			while index < length
			{
				if hit_mask & (1 << index) != 0
				{
					let value = *unsafe { data.get_unchecked(index) };
					result_handler.key_found(keys, index, Value::from_usize(value))
				}
				else
				{
					result_handler.key_not_present(keys, index)
				}
				
				index += 1;
			}
		}
		else
		{
			match result
			{
				NegativeE::EINVAL => panic!("Parameters are invalid for rte_hash_lookup_bulk_data"),
				unknown @ _ => panic!("Unknown error '{}' from rte_hash_lookup_bulk_data", unknown)
			}
		}
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_hash
	{
		self.0.handle()
	}
}
