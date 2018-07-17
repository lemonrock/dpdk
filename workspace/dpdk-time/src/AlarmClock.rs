// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A one-shot alarm.
///
/// Can be cancelled before it goes off.
///
/// Dropping this object ***DOES NOT*** cancel the alarm clock.
#[derive(Debug)]
pub struct AlarmClock<S: SmartPointer>(NonNull<S::Target>, PhantomData<S>) where S::Target: AlarmClockCallback + Sized;

impl<S: SmartPointer> AlarmClock<S> where S::Target: AlarmClockCallback + Sized
{
	/// Creates a new alarm clock.
	///
	/// The alarm clock callback will be called on or just after the `period`, but never earlier.
	///
	/// Returns `alarm_clock_callback` if an error occurred.
	#[inline(always)]
	pub fn new(period: Microseconds, alarm_clock_callback: S) -> Result<Self, S>
	{
		let callback_argument = S::to_non_null(alarm_clock_callback);
		
		match unsafe { rte_eal_alarm_set(period.into(), Self::callback, callback_argument.as_ptr() as *mut c_void) }
		{
			0 => Ok(AlarmClock(callback_argument, PhantomData)),
			
			valid if valid.is_negative() => Err(S::from_non_null(callback_argument)),
			
			return_code @ _ =>
			{
				drop(S::from_non_null(callback_argument));
				panic!("Non-negative return code '{}' from rte_eal_alarm_set()", return_code)
			}
		}
	}
	
	unsafe extern "C" fn callback(arg: *mut c_void)
	{
		debug_assert!(arg.is_not_null(), "arg is null");
		
		let alarm_clock_callback = S::from_non_null(NonNull::new_unchecked(arg as *mut S::Target));
		
		alarm_clock_callback.call();
		
		drop(alarm_clock_callback)
	}
	
	/// Cancels an alarm.
	///
	/// Can only fail if we call cancel from inside an alarm callback.
	#[inline(always)]
	pub fn cancel(self) -> Result<S, ()>
	{
		match unsafe { rte_eal_alarm_cancel(Self::callback, self.0.as_ptr() as *mut _) }
		{
			1 => Ok(S::from_non_null(self.0)),
			_ => Err(()),
		}
	}
}
