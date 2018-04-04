// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Timer<T: TimerCallback>
{
	internal: rte_timer,
	timerCallback: Option<T>,
}

impl<T: TimerCallback> Drop for Timer<T>
{
	fn drop(&mut self)
	{
		self.waitUntilTimerIsStopped()
	}
}

impl<T: TimerCallback> Timer<T>
{
	// MUST hold onto the result of this, and call waitUntilTimerIsStopped before freeing otherwise we can have an invalid memory reference for rte_timer
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn new() -> Self
	{
		let mut timer = rte_timer
		{
			expire: 0,
			sl_next: [null_mut(); 10],
			status: rte_timer_status::default(),
			period: 0,
			f: None,
			arg: null_mut(),
		};
		
		unsafe { ::dpdk_sys::rte_timer_init(&mut timer as *mut _) };
		
		Timer
		{
			internal: timer,
			timerCallback: None,
		}
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn startOrRestart(&mut self, numberOfCyclesInTicks: u64, oneOffOrPeriodic: rte_timer_type, onLogicalCore: LogicalCore, mut timerCallback: T) -> bool
	{
		let cCallbackPointer = timerCallback.asFunctionArgument();
		
		// Free previous timerCallback; we hold onto ownership of it so we can be 100% sure it doesn't get freed too soon
		self.timerCallback = None;
		self.timerCallback = Some(timerCallback);
		
		match unsafe { ::dpdk_sys::rte_timer_reset(&mut self.internal, numberOfCyclesInTicks, oneOffOrPeriodic, onLogicalCore.as_u32(), T::asFunctionPointer(), cCallbackPointer) }
		{
			0 => true,
			
			-1 => false,
			
			unexpected @ _ => panic!("Unexpected result '{}' from rte_timer_reset()", unexpected),
		}
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn waitUntilStartOrRestart(&mut self, numberOfCyclesInTicks: u64, oneOffOrPeriodic: rte_timer_type, onLogicalCore: LogicalCore, mut timerCallback: T)
	{
		let cCallbackPointer = timerCallback.asFunctionArgument();
		
		// Free previous timerCallback; we hold onto ownership of it so we can be 100% sure it doesn't get freed too soon
		self.timerCallback = None;
		self.timerCallback = Some(timerCallback);
		
		unsafe { ::dpdk_sys::rte_timer_reset_sync(&mut self.internal, numberOfCyclesInTicks, oneOffOrPeriodic, onLogicalCore.as_u32(), T::asFunctionPointer(), cCallbackPointer) }
	}
	
	#[inline(always)]
	pub fn isPending(&mut self) -> bool
	{
		isTrue(unsafe { ::dpdk_sys::rte_timer_pending(&mut self.internal) })
	}
	
	#[inline(always)]
	pub fn waitUntilTimerIsStopped(&mut self)
	{
		unsafe { ::dpdk_sys::rte_timer_stop_sync(&mut self.internal) }
	}
	
	#[inline(always)]
	pub fn stop(&mut self) -> bool
	{
		match unsafe { ::dpdk_sys::rte_timer_stop(&mut self.internal) }
		{
			0 => true,
			
			-1 => false,
			
			unexpected @ _ => panic!("Unexpected result '{}' from rte_timer_stop()", unexpected),
		}
	}
}
