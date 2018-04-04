// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct RingQueue<P, C, L>
where P: Producer, C: Consumer, L: Lifecycle
{
	ring: *mut rte_ring,
	producer: PhantomData<P>,
	consumer: PhantomData<C>,
	lifecycle: PhantomData<L>,
}

impl<P, C, L> Drop for RingQueue<P, C, L>
where P: Producer, C: Consumer, L: Lifecycle
{
	fn drop(&mut self)
	{
		L::free(self.ring)
	}
}

impl<P, C, L> RingQueue<P, C, L>
where P: Producer, C: Consumer, L: Lifecycle
{	
	const DisableWaterMarking: u32 = 0;
	
	#[inline(always)]
	pub fn new(ring: *mut rte_ring) -> Self
	{
		RingQueue
		{
			ring: ring,
			producer: PhantomData,
			consumer: PhantomData,
			lifecycle: PhantomData,
		}
	}

	#[inline(always)]
	pub fn as_ptr(&self) -> *mut rte_ring
	{
		self.ring
	}
	
	#[inline(always)]
	pub fn isFull(&self) -> bool
	{
		isTrue(unsafe { rust_rte_ring_full(self.ring) })
	}
	
	#[inline(always)]
	pub fn isEmpty(&self) -> bool
	{
		isTrue(unsafe { rust_rte_ring_empty(self.ring) })
	}
	
	#[inline(always)]
	pub fn count(&self) -> u32
	{
		unsafe { rust_rte_ring_count(self.ring) }
	}
	
	#[inline(always)]
	pub fn remaining(&self) -> u32
	{
		unsafe { rust_rte_ring_free_count(self.ring) }
	}
	
	#[inline(always)]
	pub fn disableWaterMark(&mut self)
	{
		self.setWaterMark(Self::DisableWaterMarking);
	}
	
	#[inline(always)]
	pub fn setWaterMark(&mut self, count: u32)
	{
		let result = unsafe { ::dpdk_sys::rte_ring_set_water_mark(self.ring, count) };
		if likely(result == 0)
		{
			return
		}
		
		match result
		{
			NegativeE::EINVAL => panic!("invalid watermark count, '{}' (is it greater than the ring size?)", count),
			
			unexpected @ _ => panic!("Unexpected error code '{}' from rte_ring_set_water_mark()", unexpected),
		}
	}
	
	#[inline(always)]
	pub fn dumpToStandardError(&self)
	{
		unsafe
		{
			::dpdk_sys::rte_ring_dump(stderr as *mut FILE, self.ring);
		}
	}
}
