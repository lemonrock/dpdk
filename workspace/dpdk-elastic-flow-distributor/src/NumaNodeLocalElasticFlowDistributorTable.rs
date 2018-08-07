// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents an Elastic Flow Distributor (EFD) table.
///
/// This is a perfect hash table with per-NUMA node copies for fast look ups.
///
/// ***Look ups always succeed***, returning pseudo-random rubbish if the key was never inserted.
/// This is because the underlying implementation can not track whether a key has actually been inserted.
#[derive(Debug)]
pub struct NumaNodeLocalElasticFlowDistributorTable<T: Copy + Sized>
{
	table: ElasticFlowDistributorTable<T>,
	look_up_on_numa_node: NumaNode,
	drop_prevention: Arc<ElasticFlowDistributorTable<T>>,
}

impl<T: Copy + Sized> NumaNodeLocalElasticFlowDistributorTable<T>
{
	/// Creates an instance suitable for the current hyper thread (actually, the current NUMA node).
	#[inline(always)]
	pub fn for_current_hyper_thread(table: &Arc<ElasticFlowDistributorTable<T>>) -> Self
	{
		Self
		{
			table: table.drop_unsafe_clone(),
			look_up_on_numa_node: NumaNode::numa_node_and_hyper_thread().0,
			drop_prevention: table.clone(),
		}
	}
	
	/// Looks up a key and finds its value (u8).
	///
	/// Thread safe.
	///
	/// ***Always succeeds***, returning pseudo-random rubbish if the key was never inserted.
	/// This is because the underlying implementation is a perfect hash table which can not track whether a key has actually been inserted.
	#[inline(always)]
	pub fn look_up(&self, key: &T) -> u8
	{
		self.table.look_up(self.look_up_on_numa_node, key)
	}
	
	/// Looks up several keys and finds their value (u8).
	///
	/// Thread safe.
	///
	/// ***Always succeeds***, returning pseudo-random rubbish if the keys were never inserted.
	/// This is because the underlying implementation is a perfect hash table which can not track whether a key has actually been inserted.
	#[inline(always)]
	pub fn look_up_bulk(&self, keys: &ArrayVec<[&T; LookUpBulkMaximum]>) -> ArrayVec<[u8; LookUpBulkMaximum]>
	{
		self.table.look_up_bulk(self.look_up_on_numa_node, keys)
	}
	
	/// Inserts an entry (key and value (u8)), or updates it with a new value (u8) if already present.
	///
	/// Not thread safe.
	#[inline(always)]
	pub fn insert_or_update(&self, key: &T, value: u8) -> Result<InsertionOutcome, ()>
	{
		self.table.insert_or_update(self.look_up_on_numa_node, key, value)
	}
	
	/// Removes an entry.
	///
	/// Not thread safe.
	#[inline(always)]
	pub fn remove(&self, key: &T) -> Result<(), ()>
	{
		self.table.remove(self.look_up_on_numa_node, key)
	}
	
	/// Removes an entry and returns the value (u8) associated with it.
	///
	/// Not thread safe.
	#[inline(always)]
	pub fn remove_returning_value(&self, key: &T) -> Result<u8, ()>
	{
		self.table.remove_returning_value(self.look_up_on_numa_node, key)
	}
}
