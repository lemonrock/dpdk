// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerWheel<D>(*mut tle_timer_wheel, PhantomData<D>);

impl<D> Drop for TimerWheel<D>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ::dpdk_sys::tle_timer_free(self.0) };
	}
}

impl<D> TimerWheel<D>
{
	#[inline(always)]
	pub fn create(tickSize: u32, numaSocketIdToAllocateMemoryFrom: Option<NumaSocketId>, maximumNumberOfTimers: u32, now: u64) -> Option<Self>
	{
		debug_assert!(tickSize != 0, "tickSize can not be zero");
		debug_assert!(maximumNumberOfTimers != 0, "maximumNumberOfTimers can not be zero");
		
		let mut creationArguments = tle_timer_wheel_args
		{
			tick_size: tickSize,
			socket_id: numaSocketIdToAllocateMemoryFrom.as_int32_t(),
			max_timer: maximumNumberOfTimers,
		};
		
		let result = unsafe { ::dpdk_sys::tle_timer_create(&mut creationArguments, now) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOMEM => None,
			
				E::EINVAL => panic!("Supplied an invalid value"),
			
				illegal @ _ => panic!("Unexpected errno '{}' from tle_timer_create()", illegal),
			}
		}
		else
		{
			Some(TimerWheel(result, PhantomData))
		}
	}
	
	#[inline(always)]
	pub fn start<'a>(&'a mut self, associatedData: Option<Box<D>>, interval: u64) -> Option<Timer<'a, D>>
	{
		debug_assert!(interval != 0, "interval can not be zero");
		
		if let Some(value) = associatedData
		{
			let obj = Box::into_raw(value);
			let result = self.unsafe_start(obj as *mut c_void, interval);
			if result.is_none()
			{
				let dropMe = unsafe { Box::from_raw(obj) };
				drop(dropMe);
			}
			result
		}
		else
		{
			self.unsafe_start(null_mut(), interval)
		}
	}
	
	#[inline(always)]
	fn unsafe_start<'a>(&'a mut self, obj: *mut c_void, interval: u64) -> Option<Timer<'a, D>>
	{
		let result = unsafe { ::dpdk_sys::tle_timer_start(self.0, obj, interval) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOMEM => None,
				E::ERANGE => None,
		
				E::EINVAL => panic!("Supplied an invalid value"),
		
				illegal @ _ => panic!("Unexpected errno '{}' from tle_timer_start()", illegal),
			}
		}
		else
		{
			Some(Timer(result, self))
		}
	}
	
	#[inline(always)]
	pub fn poll(&mut self, now: u64)
	{
		unsafe { ::dpdk_sys::tle_timer_expire(self.0, now) };
	}
	
	#[inline(always)]
	pub fn getExpiredTimers(&mut self) -> Vec<Option<Box<D>>>
	{
		const length: usize = 32;
		let mut timerDataObjects: Vec<*mut c_void> = Vec::with_capacity(length);

		let number = unsafe { ::dpdk_sys::tle_timer_get_expired_bulk(self.0, timerDataObjects.as_mut_ptr(), length as u32) };
		let size = number as u32 as usize; // There is a defect in the public API, whereby the number is actually an uint32_t but reported back as a int

		debug_assert!(size <= length, "size was greater than length!");

		unsafe
		{
			timerDataObjects.set_len(size);
		}
		
		timerDataObjects.into_iter().map(|x|
		{
			if x.is_null()
			{
				None
			}
			else
			{
				Some(unsafe { Box::from_raw(x as *mut D) })
			}
		}).collect()
	}
}
