// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GloballyCreatedLifecycle;

impl Lifecycle for GloballyCreatedLifecycle
{
	#[inline(always)]
	fn free(ring: *mut rte_ring)
	{
		unsafe { ::dpdk_sys::rte_ring_free(ring) }
	}
}

impl GloballyCreatedLifecycle
{
	pub fn new
	(
		name: &str, maximumCountPowerOfTwoUpto32Exclusive: u8,
		numaSocketId: Option<NumaSocketId>,
		ringQueueProducerConsumerVariant: RingQueueProducerConsumerVariant
	) -> Option<RingQueue<InefficientProducer, InefficientConsumer, GloballyCreatedLifecycle>>
	{
		#[cfg(debug_assertions)] Self::guardMaximumNameLength(name);
		
		let cName = CString::new(name).expect("GloballyCreatedLifecycle.new() name contained an interior ASCII NUL and couldn't be converted to a CString");
		
		let maximumCount: c_uint = 2 << maximumCountPowerOfTwoUpto32Exclusive as c_uint;
		
		let result = unsafe { ::dpdk_sys::rte_ring_create(cName.as_ptr(), maximumCount, numaSocketId.as_c_int(), ringQueueProducerConsumerVariant as c_uint) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E_RTE::NO_CONFIG => panic!("rte_ring_create() could not get pointer to rte_config structure"),
				E_RTE::SECONDARY => panic!("rte_ring_create() was called from a secondary process instance"),
				
				E::EINVAL => panic!("rte_ring_create(): count provided is not a power of 2; we provided maximumCount of '{}'", maximumCount),
				E::ENOSPC => panic!("rte_ring_create(): the maximum number of memzones has already been allocated"),
				E::EEXIST => panic!("rte_ring_create(): a memzone with the same name already exists"),
				E::ENOMEM => panic!("rte_ring_create(): no appropriate memory area found in which to create memzone"),
				
				unexpected @ _ => panic!("Unexpected error code '{}' from rte_ring_create()", unexpected),
			}
		}
		else
		{
			Some(RingQueue::new(result))
		}
	}
}
