// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct HashTableInner<Key: Copy + Sized + Hash, HasherType: Hasher + Default>
{
	table: NonNull<rte_hash>,
	we_should_destroy_the_table_on_drop: bool,
	marker: PhantomData<(Key, HasherType)>,
}

impl<Key: Copy + Sized + Hash, HasherType: Hasher + Default> Drop for HashTableInner<Key, HasherType>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.we_should_destroy_the_table_on_drop
		{
			unsafe { rte_hash_free(self.handle()) }
		}
	}
}

impl<Key: Copy + Sized + Hash + PartialEq, HasherType: Hasher + Default> HashTableInner<Key, HasherType>
{
	#[inline(always)]
	pub(crate) fn new(name: &CStr, capacity: usize, allocate_on: NumaNode, enable_hardware_transactional_memory_support: bool) -> Result<Self, ()>
	{
		unsafe extern "C" fn key_equality<Key: Copy + Sized + Hash + PartialEq>(key1: *const c_void, key2: *const c_void, key_len: usize) -> i32
		{
			debug_assert!(!key1.is_null(), "key1 is null");
			debug_assert!(!key2.is_null(), "key2 is null");
			debug_assert_eq!(key_len, size_of::<Key>(), "key_len is '{}' not '{}'", key_len, size_of::<Key>());
			
			let left = & * (key1 as *const Key);
			let right = & * (key2 as *const Key);
			if left.eq(right)
			{
				0
			}
			else
			{
				1
			}
		}
		
		Self::new_with_byte_wise_key_equality(name, capacity, allocate_on, enable_hardware_transactional_memory_support, false).map(|this|
		{
			unsafe { rte_hash_set_cmp_func(this.handle(), key_equality::<Key>) }
			this
		})
	}
}

impl<Key: Copy + Sized + Hash, HasherType: Hasher + Default> HashTableInner<Key, HasherType>
{
	#[inline(always)]
	pub(crate) fn new_with_byte_wise_key_equality(name: &CStr, capacity: usize, allocate_on: NumaNode, enable_hardware_transactional_memory_support: bool, enable_thread_safe_insert: bool) -> Result<Self, ()>
	{
		const HashFunctionInitialValue: u32 = 0;
		
		unsafe extern "C" fn hash<Key: Copy + Sized + Hash, HasherType: Hasher + Default>(key: *const c_void, key_len: u32, init_val: u32) -> u32
		{
			debug_assert!(!key.is_null(), "key is null");
			debug_assert_eq!(key_len, size_of::<Key>() as u32, "key_len is '{}' not '{}'", key_len, size_of::<Key>());
			debug_assert_eq!(init_val, HashFunctionInitialValue, "init_val is not '{}' but '{}'", HashFunctionInitialValue, init_val);
			
			let key = & * (key as *const Key);
			PrecomputedKeyHash::<Key, HasherType>::compute_hash(key)
		}
		
		let parameters = rte_hash_parameters
		{
			name:
			{
				debug_assert!(name.to_bytes().len() <= RTE_HASH_NAMESIZE as usize, "name without trailing NUL '{:?}' exceeds maximum length of RTE_HASH_NAMESIZE '{}'", name, RTE_HASH_NAMESIZE);
				name.as_ptr()
			},
			entries:
			{
				debug_assert_ne!(capacity, 0, "capacity can not be zero");
				debug_assert!(capacity <= RTE_HASH_ENTRIES_MAX as usize, "capacity '{}' exceeds RTE_HASH_ENTRIES_MAX '{}'", capacity, RTE_HASH_ENTRIES_MAX);
				capacity as u32
			},
			reserved: 0,
			key_len:
			{
				let key_size = size_of::<Key>();
				debug_assert!(key_size <= ::std::u32::MAX as usize, "key_size '{}' exceeds ::std::u32::MAX '{}'", key_size, ::std::u32::MAX);
				key_size as u32
			},
			hash_func: hash::<Key, HasherType>,
			hash_func_init_val: HashFunctionInitialValue,
			socket_id: allocate_on.into(),
			extra_flag:
			{
				let mut extra_flag = 0;
				
				if enable_hardware_transactional_memory_support
				{
					extra_flag |= RTE_HASH_EXTRA_FLAGS_TRANS_MEM_SUPPORT as u8;
				}
				
				if enable_thread_safe_insert
				{
					extra_flag |= RTE_HASH_EXTRA_FLAGS_MULTI_WRITER_ADD as u8;
				}
				
				extra_flag
			},
		};
		
		let result = unsafe { rte_hash_create(&parameters) };
		if unlikely!(result.is_null())
		{
			match LogicalCore::current_logical_core_error_number()
			{
				E_RTE::NO_CONFIG => panic!("rte_hash_create could not get pointer to rte_config structure"),
				E_RTE::SECONDARY => panic!("rte_hash_create was called from a secondary process instance"),
				
				E::ENOENT => panic!("rte_hash_create missing entry"),
				E::EINVAL => panic!("invalid parameter passed to rte_hash_create"),
				E::ENOSPC => panic!("rte_hash_create failed as the maximum number of memzones has already been allocated"),
				E::EEXIST => panic!("rte_hash_create failed as a memzone with the same name already exists"),
				E::ENOMEM => panic!("rte_hash_create failed as no appropriate memory area was found in which to create a memzone"),
				
				unknown @ _ => panic!("rte_hash_create failed with unknown error '{}'", unknown)
			}
		}
		else
		{
			Ok
			(
				Self
				{
					table: unsafe { NonNull::new_unchecked(result) },
					we_should_destroy_the_table_on_drop: true,
					marker: PhantomData,
				}
			)
		}
	}
	
	#[inline(always)]
	pub(crate) fn find_existing(name: &CStr) -> Option<Self>
	{
		debug_assert!(name.to_bytes().len() <= RTE_HASH_NAMESIZE as usize, "name without trailing NUL '{:?}' exceeds maximum length of RTE_HASH_NAMESIZE '{}'", name, RTE_HASH_NAMESIZE);
		
		let result = unsafe { rte_hash_find_existing(name.as_ptr()) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					table: unsafe { NonNull::new_unchecked(result) },
					we_should_destroy_the_table_on_drop: false,
					marker: PhantomData,
				}
			)
		}
	}
	
	#[inline(always)]
	pub(crate) fn clear(&self)
	{
		unsafe { rte_hash_reset(self.handle()) }
	}
	
	#[inline(always)]
	pub(crate) fn remove(&self, key: &Key) -> Option<usize>
	{
		let result = unsafe { rte_hash_del_key(self.handle(), key as *const Key as *const _) };
		if result >= 0
		{
			Some(result as usize)
		}
		else
		{
			match result
			{
				E::ENOENT => None,
				E::EINVAL => panic!("Incorrect parameters to rte_hash_del_key"),
				
				unknown @ _ => panic!("rte_hash_del_key failed with unknown error '{}'", unknown)
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn remove_with_precomputed_key_hash(&self, key: &Key, precomputed_key_hash: PrecomputedKeyHash<Key, HasherType>) -> Option<usize>
	{
		let result = unsafe { rte_hash_del_key_with_hash(self.handle(), key as *const Key as *const _, precomputed_key_hash.0) };
		if result >= 0
		{
			Some(result as usize)
		}
		else
		{
			match result
			{
				E::ENOENT => None,
				E::EINVAL => panic!("Incorrect parameters to rte_hash_del_key"),
				
				unknown @ _ => panic!("rte_hash_del_key failed with unknown error '{}'", unknown)
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn handle(&self) -> *mut rte_hash
	{
		self.table.as_ptr()
	}
}
