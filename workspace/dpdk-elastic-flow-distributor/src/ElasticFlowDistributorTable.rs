// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents an Elastic Flow Distributor (EFD) table.
///
/// This is a perfect hash table with per-NUMA node copies for fast look ups.
///
/// ***Look ups always succeed***, returning pseudo-random rubbish if the key was never inserted.
/// This is because the underlying implementation can not track whether a key has actually been inserted.
///
/// Per-thread structures may be easier to work with; look at `NumaNodeLocalElasticFlowDistributorTable`.
#[derive(Debug)]
pub struct ElasticFlowDistributorTable<Key: Copy + Sized>
{
	table: NonNull<rte_efd_table>,
	we_should_destroy_the_table_on_drop: bool,
	marker: PhantomData<Key>,
}

impl<Key: Copy + Sized> Drop for ElasticFlowDistributorTable<Key>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.we_should_destroy_the_table_on_drop
		{
			unsafe { rte_efd_free(self.handle()) }
		}
	}
}

impl<Key: Copy + Sized> ElasticFlowDistributorTable<Key>
{
	/// Create a new instance.
	///
	/// * `name` must be 32 characters or less.
	/// * `capacity` can not exceed 2^32 - 1.
	/// The first entry in `usable_on_numa_nodes` is used to store the 'offline' data of the table, ie is used for most memory allocation.
	#[inline(always)]
	pub fn new(name: &CStr, capacity: usize, usable_on_numa_nodes: IndexSet<NumaNode>) -> Result<Arc<Self>, ()>
	{
		debug_assert!(name.to_bytes().len() <= RTE_EFD_NAMESIZE as usize, "name without trailing NUL '{:?}' exceeds maximum length of RTE_EFD_NAMESIZE '{}'", name, RTE_EFD_NAMESIZE);
		
		debug_assert_ne!(capacity, 0, "capacity can not be zero");
		debug_assert!(capacity <= ::std::u32::MAX as usize, "capacity '{}' exceeds ::std::u32::MAX '{}'", capacity, ::std::u32::MAX);
		
		let key_size = size_of::<Key>();
		debug_assert!(key_size <= ::std::u32::MAX as usize, "key_size '{}' exceeds ::std::u32::MAX '{}'", key_size, ::std::u32::MAX);
		
		debug_assert_ne!(usable_on_numa_nodes.len(), 0, "usable_on_numa_nodes can not be empty");
		let mut online_cpu_socket_bitmask = 0;
		for numa_node in usable_on_numa_nodes.iter()
		{
			let value: u8 = (*numa_node).into();
			online_cpu_socket_bitmask |= 1 << value
		}
		
		let offline_cpu_socket: u8 = (*usable_on_numa_nodes.get_index(0).unwrap()).into();
		
		let result = unsafe { rte_efd_create(name.as_ptr(), capacity as u32, key_size as u32, online_cpu_socket_bitmask, offline_cpu_socket) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok
			(
				Arc::new
				(
					Self
					{
						table: unsafe { NonNull::new_unchecked(result) },
						we_should_destroy_the_table_on_drop: true,
						marker: PhantomData,
					}
				)
			)
		}
	}
	
	/// Creates an instance suitable for the current hyper thread (actually, the current NUMA node).
	#[inline(always)]
	pub fn for_current_hyper_thread(this: &Arc<Self>) -> NumaNodeLocalElasticFlowDistributorTable<Key>
	{
		NumaNodeLocalElasticFlowDistributorTable::for_current_hyper_thread(this)
	}
	
	/// Find an existing instance.
	///
	/// * `name` must be 32 characters or less.
	#[inline(always)]
	pub fn find_existing(name: &CStr) -> Option<Self>
	{
		debug_assert!(name.to_bytes().len() <= RTE_EFD_NAMESIZE as usize, "name without trailing NUL '{:?}' exceeds maximum length of RTE_EFD_NAMESIZE '{}'", name, RTE_EFD_NAMESIZE);
		
		let result = unsafe { rte_efd_find_existing(name.as_ptr()) };
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
	
	/// Looks up a key and finds its value (u8).
	///
	/// Thread safe.
	///
	/// ***Always succeeds***, returning pseudo-random rubbish if the key was never inserted.
	/// This is because the underlying implementation is a perfect hash table which can not track whether a key has actually been inserted.
	///
	/// `look_up_on_numa_node` should ideally be the caller's current NumaNode, viz `NumaNode::numa_node_and_hyper_thread().0`.
	///
	/// Since this can be a touch expensive in some circumstances, a caller may want to use an NumaNodeLocalElasticFlowDistributorTable instead.
	#[inline(always)]
	pub fn look_up(&self, look_up_on_numa_node: NumaNode, key: &Key) -> u8
	{
		unsafe { rte_efd_lookup(self.handle(), look_up_on_numa_node.into(), key as *const Key as *const _) }
	}
	
	/// Looks up several keys and finds their value (u8).
	///
	/// Thread safe.
	///
	/// ***Always succeeds***, returning pseudo-random rubbish if the keys were never inserted.
	/// This is because the underlying implementation is a perfect hash table which can not track whether a key has actually been inserted.
	///
	/// `look_up_on_numa_node` should ideally be the caller's current NumaNode, viz `NumaNode::numa_node_and_hyper_thread().0`.
	///
	/// Since this can be a touch expensive in some circumstances, a caller may want to use an NumaNodeLocalElasticFlowDistributorTable instead.
	#[inline(always)]
	pub fn look_up_bulk(&self, look_up_on_numa_node: NumaNode, keys: ArrayVec<[&Key; LookUpBulkMaximum]>) -> ArrayVec<[u8; LookUpBulkMaximum]>
	{
		let mut values = ArrayVec::new();
		unsafe { rte_efd_lookup_bulk(self.handle(), look_up_on_numa_node.into(), keys.len() as i32, keys.as_ptr() as *const &Key as *const *const Key as *const *const _ as *mut *const _, values.as_mut_ptr()) }
		unsafe { values.set_len(keys.len()) };
		values
	}
	
	/// Inserts an entry (key and value (u8)), or updates it with a new value (u8) if already present.
	///
	/// Not thread safe.
	///
	/// `look_up_on_numa_node` should ideally be the caller's current NumaNode, viz `NumaNode::numa_node_and_hyper_thread().0`.
	///
	/// Since this can be a touch expensive in some circumstances, a caller may want to use an NumaNodeLocalElasticFlowDistributorTable instead.
	#[inline(always)]
	pub fn insert_or_update(&self, look_up_on_numa_node: NumaNode, key: &Key, value: u8) -> Result<InsertionOutcome, ()>
	{
		let result = unsafe { rte_efd_update(self.handle(), look_up_on_numa_node.into(), key as *const Key as *const _, value) };
		
		use self::InsertionOutcome::*;
		
		match result as u32
		{
			0 => Ok(Inserted { group_is_now_full: false }),
			
			RTE_EFD_UPDATE_WARN_GROUP_FULL => Ok(Inserted { group_is_now_full: true }),
			
			RTE_EFD_UPDATE_NO_CHANGE => Ok(Unchanged),
			
			RTE_EFD_UPDATE_FAILED => Err(()),
			
			unknown @ _ => panic!("Unknown error '{}'", unknown)
		}
	}
	
	/// Removes an entry.
	///
	/// Not thread safe.
	///
	/// `look_up_on_numa_node` should ideally be the caller's current NumaNode, viz `NumaNode::numa_node_and_hyper_thread().0`.
	///
	/// Since this can be a touch expensive in some circumstances, a caller may want to use an NumaNodeLocalElasticFlowDistributorTable instead.
	#[inline(always)]
	pub fn remove(&self, look_up_on_numa_node: NumaNode, key: &Key) -> Result<(), ()>
	{
		let result = unsafe { rte_efd_delete(self.handle(), look_up_on_numa_node.into(), key as *const Key as *const _, null_mut()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}
	
	/// Removes an entry and returns the value (u8) associated with it.
	///
	/// Not thread safe.
	///
	/// `look_up_on_numa_node` should ideally be the caller's current NumaNode, viz `NumaNode::numa_node_and_hyper_thread().0`.
	///
	/// Since this can be a touch expensive in some circumstances, a caller may want to use an NumaNodeLocalElasticFlowDistributorTable instead.
	#[inline(always)]
	pub fn remove_returning_value(&self, look_up_on_numa_node: NumaNode, key: &Key) -> Result<u8, ()>
	{
		let mut value = unsafe { uninitialized() };
		
		let result = unsafe { rte_efd_delete(self.handle(), look_up_on_numa_node.into(), key as *const Key as *const _, &mut value) };
		if likely!(result == 0)
		{
			Ok(value)
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_efd_table
	{
		self.table.as_ptr()
	}
	
	#[inline(always)]
	fn drop_unsafe_clone(&self) -> Self
	{
		Self
		{
			table: self.table,
			we_should_destroy_the_table_on_drop: false,
			marker: PhantomData,
		}
	}
}
