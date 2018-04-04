// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


const MaximumNumberOfElements: usize = ETH_RSS_RETA_SIZE_512 as usize / RTE_RETA_GROUP_SIZE;

#[repr(C, packed)]
#[allow(missing_debug_implementations)]
#[allow(missing_copy_implementations)]
pub struct ReceiveSideScalingRetaIndirectionTable
{
	size: PowerOfTwoSixteenBit,
	entries: [rte_eth_rss_reta_entry64; MaximumNumberOfElements],
}

impl ReceiveSideScalingRetaIndirectionTable
{
	pub fn empty(size: PowerOfTwoSixteenBit) -> Self
	{
		ReceiveSideScalingRetaIndirectionTable
		{
			size: size,
			entries: [rte_eth_rss_reta_entry64::default(); MaximumNumberOfElements],
		}
	}
	
	pub fn new(size: PowerOfTwoSixteenBit, numberOfReceiveThenTransmitQueuePairs: u16) -> Self
	{
		debug_assert!(numberOfReceiveThenTransmitQueuePairs < size as u16, "numberOfReceiveThenTransmitQueuePairs '{}' equals or exceeds size '{}'", numberOfReceiveThenTransmitQueuePairs, size as u16);
		
		let table = Self::empty(size);
		
		let numberOfEntries = size as u16;
		
		for index in 0..numberOfEntries
		{
			table.enableReceiveQueue(index as usize, (index % numberOfReceiveThenTransmitQueuePairs) as QueueIdentifier);
		}
		
		table
	}
	
	#[inline(always)]
	pub fn enableReceiveQueue(&self, index: usize, receiveQueueIdentifier: QueueIdentifier)
	{
		debug_assert!(index < self.size as usize, "index '{}' equals or exceeds size of '{}'", index, self.size as usize);
		debug_assert!(receiveQueueIdentifier < MaximumReceiveQueues as u16, "receiveQueueIdentifier '{} equals or exceeds MaximumReceiveQueues of '{}'", receiveQueueIdentifier, MaximumReceiveQueues);
		
		let mut entry = self.entries[index / RTE_RETA_GROUP_SIZE];
		let offset = index % RTE_RETA_GROUP_SIZE;
		
		entry.mask = entry.mask | 1 << (offset as u64);
		entry.reta[offset] = receiveQueueIdentifier;
	}
	
	#[inline(always)]
	pub fn as_rte_eth_rss_reta_entry64(&mut self) -> *mut rte_eth_rss_reta_entry64
	{
		self.entries.as_mut_ptr()
	}
	
	#[inline(always)]
	pub fn retaSize(&self) -> u16
	{
		self.size as u16
	}
}
