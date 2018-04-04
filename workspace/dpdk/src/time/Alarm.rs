// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// WARNING: If the Alarm goes out of scope and is dropped, then the pointers passed to C will be come invalid. Be careful!
/// We could investigate move'ing (eg use mut self, not &mut self), mem::forget and into_raw / from_raw a la CString
pub trait Alarm : MutableCallback1<()>
{
	/// One shot alarms - not like cron!
	/// May be called on or after toGoOffInHowManyMicroseconds, but not before
	/// Returns true if alarm set
	/// It is not required to call cancel() or cancelAll() from inside call(), but it is possible if so desired (each to cancel further alarms of the same configuration)
	#[inline(always)]
	fn set(&mut self, toGoOffInHowManyMicroseconds: u64) -> bool
	{
		let result = unsafe { ::dpdk_sys::rte_eal_alarm_set(toGoOffInHowManyMicroseconds, Self::asFunctionPointer(), self.asFunctionArgument()) };
		if likely(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				valid if valid.is_negative() => false,
			
				_ => panic!("Non-negative return code '{}' from rte_eal_alarm_set()", result),
			}
		}
	}
	
	/// (usize, bool) => (how many cancelled, more possible)
	#[inline(always)]
	fn cancel(&mut self) -> (usize, bool)
	{
		let functionArgument = self.asFunctionArgument();
		
		Self::cancelInternal(functionArgument)
	}
	
	/// (usize, bool) => (how many cancelled, more possible)
	#[inline(always)]
	fn cancelAll() -> (usize, bool)
	{
		const functionArgument: *mut c_void = (-1 as isize) as *mut c_void;
		
		Self::cancelInternal(functionArgument)
	}
	
	/// (usize, bool) => (how many cancelled, more possible)
	#[inline(always)]
	fn cancelInternal(functionArgument: *mut c_void) -> (usize, bool)
	{
		unsafe { rust_rte_reset_errno() };
		
		match unsafe { ::dpdk_sys::rte_eal_alarm_cancel(Self::asFunctionPointer(), functionArgument) }
		{
			result if result > 0 =>
			{
				match unsafe { rust_rte_errno() }
				{
					0 => (result as usize, false),
					
					E::EINPROGRESS => (result as usize, true),
					
					unexpected @ _ => panic!("Invalid error code '{}' from rte_eal_alarm_cancel() when result was greater than zero '{}'", unexpected, result),
				}
			},
			
			0 =>
			{
				match unsafe { rust_rte_errno() }
				{
					// Seems unlikely, not described in documentation, but might occur. Net effect is the same as E:ENOENT
					0 => (0, false),
					
					E::EINPROGRESS => (0, true),
					E::ENOENT => (0, false),
					
					unexpected @ _ => panic!("Invalid error code '{}' from rte_eal_alarm_cancel() when result was zero", unexpected),
				}
			}
			
			-1 =>
			{
				match unsafe { rust_rte_errno() }
				{
					E::EINVAL => panic!("Invalid parameter - NULL callback"),
					
					unexpected @ _ => panic!("Invalid error code '{}' from rte_eal_alarm_cancel() when return code was -1", unexpected),
				}
			},
			
			unexpected @ _ => panic!("Invalid return code '{}' from rte_eal_alarm_cancel()", unexpected),
		}
	}
}
