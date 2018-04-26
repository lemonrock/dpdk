// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Do not adjust the number of fields in this struct without changing the implementation of `Self::callback`.
/// A timer which calls a callback when it expires (goes off).
///
/// Dropping this object ***DOES*** cancel the timer.
#[derive(Debug)]
pub struct Timer<T: TimerCallback>(UnsafeCell<rte_timer>, PhantomData<T>);

impl<T: TimerCallback> Drop for Timer<T>
{
	fn drop(&mut self)
	{
		if !self.0.arg.is_null()
		{
			self.wait_until_timer_is_stopped();
			drop(Arc::from_raw(self.0.arg as *const T))
		}
	}
}

impl<T: TimerCallback> Timer<T>
{
	/// This must be called before using any code that uses HPET or timer functionality.
	#[inline(always)]
	pub fn initialize()
	{
		unsafe
		{
			rte_timer_subsystem_init();
			
			match rte_eal_hpet_init()
			{
				0 => (),
				
				-1 => panic!("HPET is not available"),
				
				unexpected @ _ => panic!("HPET initialisation returned unexpected error code '{}'", unexpected),
			}
		}
	}
	
	/// Create a new timer.
	#[inline(always)]
	pub fn new() -> Box<Self>
	{
		let this = Box::new
		(
			Timer
			(
				UnsafeCell::new
				(
					rte_timer
					{
						expire: 0,
						sl_next: [null_mut(); 10],
						status: unsafe { uninitialized() },
						period: 0,
						f: unsafe { uninitialized() },
						arg: null_mut(),
					}
				),
				PhantomData
			)
		);
		
		unsafe { rte_timer_init(&mut this.0) };
		
		this
	}
	
	unsafe extern "C" fn callback(arg1: *mut rte_timer, arg2: *mut c_void)
	{
		debug_assert!(arg1.is_not_null(), "arg1 is null");
		debug_assert!(arg2.is_not_null(), "arg2 is null");
		debug_assert_eq!(arg2, unsafe { & * arg1 }.arg, "arg2 and arg1.arg are not the same");
		
		let timer_callback = unsafe { & * (arg2 as *mut T) };
		
		// Possible as long as there is only one non-zero-sized field in Self.
		let this = unsafe { & * (arg1 as *const Self) };
		
		timer_callback.call(this)
	}
	
	/// Uses HPET timer where possible.
	///
	/// If in a running or configuring state, then returns an `Err` which contains the `timer_callback` passed in.
	#[inline(always)]
	pub fn non_blocking_start_or_restart(&self, period: Cycles, one_off_or_periodic: rte_timer_type, run_callback_on_logical_core: LogicalCore, timer_callback: Arc<T>) -> Result<(), Arc<T>>
	{
		let callback_argument = Arc::into_raw(timer_callback.clone()) as *mut T;
		match unsafe { rte_timer_reset(self.0.get(), period.into(), one_off_or_periodic, run_callback_on_logical_core.as_u32(), Self::callback, callback_argument as *mut c_void) }
		{
			0 => Ok(()),
			
			-1 => Err(Arc::from_raw(callback_argument as *const T)),
			
			unexpected @ _ =>
			{
				drop(Arc::from_raw(callback_argument as *const T));
				panic!("Unexpected result '{}' from rte_timer_reset()", unexpected)
			},
		}
	}
	
	/// Uses HPET timer where possible.
	#[inline(always)]
	pub fn blocking_start_or_restart(&self, period: Cycles, one_off_or_periodic: rte_timer_type, run_callback_on_logical_core: LogicalCore, mut timer_callback: Arc<T>)
	{
		let callback_argument = Arc::into_raw(timer_callback.clone()) as *mut T;
		unsafe { rte_timer_reset_sync(self.0.get(), period, one_off_or_periodic, run_callback_on_logical_core.as_u32(), Self::callback(), callback_argument) }
	}
	
	/// Is pending.
	///
	/// If so, the timer has not yet been fired and can be restarted or stopped using non-blocking behaviour without any waiting.
	#[inline(always)]
	pub fn is_pending(&self) -> bool
	{
		isTrue(unsafe { rte_timer_pending(self.0.get()) })
	}
	
	/// Stop.
	#[inline(always)]
	pub fn non_blocking_stop(&self) -> Result<(), ()>
	{
		match unsafe { rte_timer_stop(self.0.get()) }
		{
			0 => Ok(()),
			
			-1 => Err(()),
			
			unexpected @ _ => panic!("Unexpected result '{}' from rte_timer_stop()", unexpected),
		}
	}
	
	/// Stop.
	#[inline(always)]
	pub fn blocking_stop(&self)
	{
		unsafe { rte_timer_stop_sync(self.0.get()) }
	}
}
