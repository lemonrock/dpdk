// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Producer
{
	#[inline(always)]
	fn producer(ring: *mut rte_ring, object: *mut c_void) -> Result<(), ProducerError>;
	
	#[inline(always)]
	fn producerBulk(ring: *mut rte_ring, table: *mut *const c_void, count: u32) -> Result<(), ProducerError>;
	
	#[inline(always)]
	fn producerBurst(ring: *mut rte_ring, table: *mut *const c_void, count: u32) -> u32;

	#[inline(always)]
	fn processProducerResult(result: c_int) -> Result<(), ProducerError>
	{
		if likely(result == 0)
		{
			return Ok(())
		}
		else
		{
			match result
			{
				NegativeE::EDQUOT => Err(ProducerError::HighWaterMarkExceeded),
				NegativeE::ENOBUFS => Err(ProducerError::NoRoomAvailableAndNothingProducerd),
			
				unexpected @ _ => panic!("Unexpected error code '{}' from ring queue producer operation ()", unexpected),
			}
		}
	}
}
