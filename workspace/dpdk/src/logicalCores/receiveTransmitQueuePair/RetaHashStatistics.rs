// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
#[repr(packed)]
pub struct RetaHashStatistics<A: Array<AtomicU64>>
{
	counters: A,
}

impl<A: Array<AtomicU64>> Default for RetaHashStatistics<A>
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl<A: 'static + Array<AtomicU64>> RetaHashStatistics<A>
{	
	#[inline(always)]
	pub fn incrementFrom_rte_mbuf_rss(&mut self, rss: u32)
	{
		let index = rss as usize & A::mask();
		let counter = unsafe { self.counters.get_unchecked_mut(index) };
		counter.fetch_add(1, Ordering::Relaxed);
	}
	
	// If we re-distribute traffic processing, then we need to pause all pairs so that TCP (or even UDP data structures, eg in the event of QUIC) can be redistributed
}
