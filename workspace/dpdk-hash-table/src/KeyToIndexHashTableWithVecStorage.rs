// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Always allocated on current hyper thread's NUMA node.
#[derive(Debug)]
pub struct KeyToIndexHashTableWithVecStorage<Key: Copy + Sized + Hash, Value, HasherType: Hasher + Default>
{
	table: KeyToIndexHashTable<Key, HasherType>,
	storage: Vec<Option<Value>>,
}

impl<Key: Copy + Sized + Hash + PartialEq, Value, HasherType: Hasher + Default> KeyToIndexHashTableWithVecStorage<Key, Value, HasherType>
{
	/// Create a new instance, using Key's implementation of the `PartialEq` trait.
	///
	/// * `name` must be 32 characters or less.
	/// * `capacity` can not exceed 1,073,741,824 (`RTE_HASH_ENTRIES_MAX`).
	/// * `enable_hardware_transactional_memory_support` will only work on Hashwell, Skylake and later CPUs with suitable firmware fixes; it may be implemented in the future on ARM and PowerPC CPUs.
	/// * `enable_thread_safe_insert` always insertion to be thread safe. If `enable_hardware_transactional_memory_support` is specified then thread safe insertion is optimized with hardware transactional memory usage.
	#[inline(always)]
	pub fn new(name: &CStr, capacity: usize, enable_hardware_transactional_memory_support: bool) -> Result<Self, ()>
	{
		KeyToIndexHashTable::new(name, capacity, Self::allocate_on(), enable_hardware_transactional_memory_support).map(|table| KeyToIndexHashTableWithVecStorage
		{
			table,
			storage: Self::default_storage(capacity),
		})
	}
}

impl<Key: Copy + Sized + Hash, Value: Clone, HasherType: Hasher + Default> KeyToIndexHashTableWithVecStorage<Key, Value, HasherType>
{
	/// Removes an entry.
	///
	/// Not thread safe, even if `Self::new_with_byte_wise_key_equality` was called with `enable_thread_safe_insert` set to `true`.
	///
	/// Returns `None` if the key was not present.
	///
	/// Returns `Some(value)` if the key was present.
	#[inline(always)]
	pub fn remove_returning_value(&mut self, key: &Key) -> Option<Value>
	{
		match self.table.remove(key)
		{
			None => None,
			Some(index) =>
				{
					let slot = unsafe { self.storage.get_unchecked_mut(index) };
					let value = slot.clone();
					*slot = None;
					value
				}
		}
	}
	
	/// Removes an entry, using a precomputed hash (might be occasionally more optimal).
	///
	/// Not thread safe, even if `Self::new_with_byte_wise_key_equality` was called with `enable_thread_safe_insert` set to `true`.
	///
	/// Returns `None` if the key was not present.
	///
	/// Returns `Some(index)` if the key was present.
	#[inline(always)]
	pub fn remove_returning_value_with_precomputed_key_hash(&mut self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Option<Value>
	{
		match self.table.remove_with_precomputed_key_hash(key, precomputed_key_hash)
		{
			None => None,
			Some(index) =>
				{
					let slot = unsafe { self.storage.get_unchecked_mut(index) };
					let value = slot.clone();
					*slot = None;
					value
				}
		}
	}
}

impl<Key: Copy + Sized + Hash, Value, HasherType: Hasher + Default> KeyToIndexHashTableWithVecStorage<Key, Value, HasherType>
{
	/// Create a new instance, with Key equality being byte-wise.
	///
	/// * `name` must be 32 characters or less.
	/// * `capacity` can not exceed 1,073,741,824 (`RTE_HASH_ENTRIES_MAX`).
	/// * `enable_hardware_transactional_memory_support` will only work on Hashwell, Skylake and later CPUs with suitable firmware fixes; it may be implemented in the future on ARM and PowerPC CPUs.
	/// * `enable_thread_safe_insert` always insertion to be thread safe. If `enable_hardware_transactional_memory_support` is specified then thread safe insertion is optimized with hardware transactional memory usage.
	#[inline(always)]
	pub fn new_with_byte_wise_key_equality(name: &CStr, capacity: usize, enable_hardware_transactional_memory_support: bool, enable_thread_safe_insert: bool) -> Result<Self, ()>
	{
		KeyToIndexHashTable::new_with_byte_wise_key_equality(name, capacity, Self::allocate_on(), enable_hardware_transactional_memory_support, enable_thread_safe_insert).map(|table| KeyToIndexHashTableWithVecStorage
		{
			table,
			storage: Self::default_storage(capacity),
		})
	}
	
	#[inline(always)]
	fn allocate_on() -> NumaNode
	{
		NumaNode::numa_node_and_hyper_thread().0
	}
	
	#[inline(always)]
	fn default_storage(capacity: usize) -> Vec<Option<Value>>
	{
		let mut storage = Vec::with_capacity(capacity);
		for _ in 0 .. capacity
		{
			storage.push(None)
		}
		storage
	}
	
	/// Clears (resets) this this instance with initial values.
	#[inline(always)]
	pub fn clear(&self)
	{
		self.table.clear()
	}
	
	/// Inserts a key-value entry.
	///
	/// Not obvious what underlying DPDK implementation does with duplicate keys.
	///
	/// Returns an error if out of space (ie over capacity).
	///
	/// Only multi-thread safe if constructed with `new_with_byte_wise_key_equality` and `enable_thread_safe_insert` was `true`.
	///
	/// Currently does a memory copy; a future design could provide placement.
	#[inline(always)]
	pub fn insert(&mut self, key: Key, value: Value) -> Result<&mut Value, ()>
	{
		match self.table.insert(key)
		{
			Err(()) => Err(()),
			Ok(index) =>
			{
				let slot = unsafe { self.storage.get_unchecked_mut(index) };
				*slot = Some(value);
				slot.as_mut().ok_or(())
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
	pub fn insert_with_precomputed_key_hash(&mut self, key: Key, value: Value, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Result<&mut Value, ()>
	{
		match self.table.insert_with_precomputed_key_hash(key, precomputed_key_hash)
		{
			Err(()) => Err(()),
			Ok(index) =>
			{
				let slot = unsafe { self.storage.get_unchecked_mut(index) };
				*slot = Some(value);
				slot.as_mut().ok_or(())
			}
		}
	}
	
	/// Removes an entry.
	///
	/// Not thread safe, even if `Self::new_with_byte_wise_key_equality` was called with `enable_thread_safe_insert` set to `true`.
	///
	/// Returns `false` if the key was not present.
	///
	/// Returns `true` if the key was present.
	#[inline(always)]
	pub fn remove(&mut self, key: &Key) -> bool
	{
		match self.table.remove(key)
		{
			None => false,
			Some(index) =>
			{
				let slot = unsafe { self.storage.get_unchecked_mut(index) };
				*slot = None;
				true
			}
		}
	}
	
	/// Removes an entry, using a precomputed hash (might be occasionally more optimal).
	///
	/// Not thread safe, even if `Self::new_with_byte_wise_key_equality` was called with `enable_thread_safe_insert` set to `true`.
	///
	/// Returns `false` if the key was not present.
	///
	/// Returns `true` if the key was present.
	#[inline(always)]
	pub fn remove_with_precomputed_key_hash(&mut self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> bool
	{
		match self.table.remove_with_precomputed_key_hash(key, precomputed_key_hash)
		{
			None => false,
			Some(index) =>
			{
				let slot = unsafe { self.storage.get_unchecked_mut(index) };
				*slot = None;
				true
			}
		}
	}
	
	/// Finds an entry.
	///
	/// Always thread safe.
	///
	/// Returns `None` if the key was not present.
	///
	/// Returns `Some(index)` if the key was present.
	#[inline(always)]
	pub fn look_up(&self, key: &Key) -> Option<&Value>
	{
		match self.table.look_up(key)
		{
			None => None,
			Some(index) => unsafe { self.storage.get_unchecked(index).as_ref() }
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
	pub fn look_up_with_precomputed_key_hash(&self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Option<&Value>
	{
		match self.table.look_up_with_precomputed_key_hash(key, precomputed_key_hash)
		{
			None => None,
			Some(index) => unsafe { self.storage.get_unchecked(index).as_ref() }
		}
	}
	
//	/// Finds entries.
//	///
//	/// Always thread safe.
//	#[inline(always)]
//	pub fn look_up_bulk(&self, keys: &ArrayVec<[&Key; LookUpBulkMaximum]>, mut result_handler: impl LookUpBulkResultHandler<Key, usize>)
//	{
//	}
//
//	/// Iterate over key-value pairs.
//	#[inline(always)]
//	pub fn iterate<'a>(&'a self) -> KeyToIndexHashTableIterator<'a, Key, HasherType>
//	{
//	}
}
