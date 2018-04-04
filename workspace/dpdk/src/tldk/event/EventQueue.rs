// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventQueue(*mut tle_evq);

impl Drop for EventQueue
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ::dpdk_sys::tle_evq_destroy(self.0) }
	}
}

impl EventQueue
{
	#[inline(always)]
	pub fn create(numaSocketIdToAllocateMemoryFrom: Option<NumaSocketId>, maximumNumberOfEvents: u32) -> Option<EventQueue>
	{
		let parameters = tle_evq_param
		{
			socket_id: numaSocketIdToAllocateMemoryFrom.as_int32_t(),
			max_events: maximumNumberOfEvents,
		};
		
		let result = unsafe { ::dpdk_sys::tle_evq_create(&parameters) };

		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOMEM => None,
			
				E::EINVAL => panic!("Supplied an invalid value"),
			
				illegal @ _ => panic!("Unexpected errno '{}' from tle_evq_create()", illegal),
			}
		}
		else
		{
			Some(EventQueue(result))
		}
	}
	
	// #[inline(always)]
	// pub fn idle()
	// {
	// 	// fn rust_tle_evq_idle(evq: *mut tle_evq as "struct tle_evq *", ev: *mut *mut tle_event as "struct tle_event **", num: uint32_t as "uint32_t")
	// 	unsafe { ::dpdk_sys::rust_tle_evq_idle() }
	// }
	//
	// #[inline(always)]
	// pub fn get()
	// {
	// 	// fn rust_tle_evq_get(evq: *mut tle_evq as "struct tle_evq *", evd: *mut *const c_void as "const void **", num: uint32_t as "uint32_t") -> uint32_t as "uint32_t"
	// 	unsafe { ::dpdk_sys::rust_tle_evq_get() }
	// }
}
