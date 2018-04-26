// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A one-shot alarm.
///
/// Can be cancelled before it goes off.
///
/// Dropping this object ***DOES NOT*** cancel the alarm clock.
#[derive(Debug)]
pub struct AlarmClock<T: AlarmClockCallback>(NonNull<T>);

impl<T: AlarmClockCallback> AlarmClock<T>
{
	/// Creates a new alarm clock.
	///
	/// The alarm clock callback will be called on or just after the `period`, but never earlier.
	///
	/// Returns `alarm_clock_callback` if an error occurred.
	#[inline(always)]
	pub fn new(period: Microseconds, alarm_clock_callback: Box<T>) -> Result<Self, Box<T>>
	{
		let callback_argument = Box::into_raw(alarm_clock_callback);
		
		match unsafe { rte_eal_alarm_set(period.into() as u64, Self::callback, callback_argument as *mut c_void) }
		{
			0 => Ok(AlarmClock(unsafe { NonNull::new_unchecked(callback_argument) })),
			
			valid if valid.is_negative() => Err(unsafe { Box::from_raw(callback_argument) }),
			
			_ =>
			{
				drop(unsafe { Box::from_raw(callback_argument) });
				panic!("Non-negative return code '{}' from rte_eal_alarm_set()", result)
			}
		}
		
		this
	}
	
	unsafe extern "C" fn callback(arg: *mut c_void)
	{
		debug_assert!(arg.is_not_null(), "arg is null");
		
		let alarm_clock_callback = unsafe { Box::from_raw(arg as *mut T) };
		
		alarm_clock_callback.call();
		
		drop(alarm_clock_callback)
	}
	
	/// Cancels an alarm.
	///
	/// Can only fail if we call cancel from inside an alarm callback.
	#[inline(always)]
	pub fn cancel(self) -> Result<Box<T>, ()>
	{
		match unsafe { rte_eal_alarm_cancel(Self::callback, self.0.as_ptr()) }
		{
			1 => Ok(unsafe { Box::from_raw(self.0.as_ptr()) }),
			_ => Err(()),
		}
	}
}
