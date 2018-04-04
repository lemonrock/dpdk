// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Consumer
{
	fn dequeue(ring: *mut rte_ring, intoConsumerdObjectHolder: *mut *mut c_void) -> bool;
	
	fn dequeueBulk(ring: *mut rte_ring, table: *mut *mut c_void, count: u32) -> bool;
	
	fn dequeueBurst(ring: *mut rte_ring, table: *mut *mut c_void, count: u32) -> u32;
	
	#[inline(always)]
	fn processConsumerResult(result: c_int) -> bool
	{
		if likely(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				NegativeE::ENOENT => false,
			
				unexpected @ _ => panic!("Unexpected error code '{}' from ring queue dequeue operation ()", unexpected),
			}
		}
	}
}
