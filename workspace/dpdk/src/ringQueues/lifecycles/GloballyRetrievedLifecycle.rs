// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GloballyRetrievedLifecycle;

impl Lifecycle for GloballyRetrievedLifecycle
{
	#[allow(unused_variables)]
	#[inline(always)]
	fn free(ring: *mut rte_ring)
	{
	}
}

impl GloballyRetrievedLifecycle
{
	pub fn lookup(name: &str) -> Option<RingQueue<InefficientProducer, InefficientConsumer, GloballyRetrievedLifecycle>>
	{
		#[cfg(debug_assertions)] Self::guardMaximumNameLength(name);
		
		let cName = CString::new(name).expect("GloballyRetrievedLifecycle.lookup() name contained an interior ASCII NUL and couldn't be converted to a CString");
		
		let result = unsafe { ::dpdk_sys::rte_ring_lookup(cName.as_ptr()) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOENT => None,
				
				unexpected @ _ => panic!("Unexpected error code '{}' from rte_ring_lookup()", unexpected),
			}
		}
		else
		{
			Some(RingQueue::new(result))
		}
	}
		
	#[inline(always)]
	pub fn dumpAllToStandardError()
	{
		unsafe
		{
			::dpdk_sys::rte_ring_list_dump(stderr as *mut FILE);
		}
	}
}
