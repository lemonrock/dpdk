// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SingleConsumer;

impl Consumer for SingleConsumer
{
	// Consumers one object. Suboptimal as default implementation. Returns true if an object was dequeued into intoConsumerdObjectHolder
	#[inline(always)]
	fn dequeue(ring: *mut rte_ring, intoConsumerdObjectHolder: *mut *mut c_void) -> bool
	{
		Self::processConsumerResult(unsafe { rust_rte_ring_sc_dequeue(ring, intoConsumerdObjectHolder) })
	}
	
	// Consumers zero or more objects. Suboptimal as default implementation. Returns true if one or more objects was dequeued into table
	#[inline(always)]
	fn dequeueBulk(ring: *mut rte_ring, table: *mut *mut c_void, count: u32) -> bool
	{
		Self::processConsumerResult(unsafe { rust_rte_ring_sc_dequeue_bulk(ring, table, count) })
	}
	
	// Takes number of objects to dequeue. Returns number of objects dequeued. Suboptimal as default implementation.
	#[inline(always)]
	fn dequeueBurst(ring: *mut rte_ring, table: *mut *mut c_void, count: u32) -> u32
	{
		unsafe { rust_rte_ring_sc_dequeue_burst(ring, table, count) }
	}
}
