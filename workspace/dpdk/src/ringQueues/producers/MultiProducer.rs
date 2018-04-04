// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct MultiProducer;

impl Producer for MultiProducer
{
	// Producers one object.
	#[inline(always)]
	fn producer(ring: *mut rte_ring, object: *mut c_void) -> Result<(), ProducerError>
	{
		Self::processProducerResult(unsafe { rust_rte_ring_mp_enqueue(ring, object) })
	}
	
	// Takes number of objects to producer.
	#[inline(always)]
	fn producerBulk(ring: *mut rte_ring, table: *mut *const c_void, count: u32) -> Result<(), ProducerError>
	{
		Self::processProducerResult(unsafe { rust_rte_ring_mp_enqueue_bulk(ring, table, count) })
	}
	
	// Takes number of objects to producer. Returns number of objects producerd.
	#[inline(always)]
	fn producerBurst(ring: *mut rte_ring, table: *mut *const c_void, count: u32) -> u32
	{
		unsafe { rust_rte_ring_mp_enqueue_burst(ring, table, count) }
	}
}
