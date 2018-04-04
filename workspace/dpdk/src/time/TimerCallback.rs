// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait TimerCallback : Sized
{
	#[inline(always)]
	fn call(&mut self, timer: &Timer<Self>);
	
	#[inline(always)]
	unsafe extern "C" fn callFromC(arg1: *mut rte_timer, arg2: *mut c_void)
	{
		let timer: &mut Timer<Self> = &mut *(arg1 as *mut Timer<Self>);
		let us: &mut Self = &mut *(arg2 as *mut Self);
		us.call(timer);
		forget(timer);
	}
	
	#[inline(always)]
	fn asFunctionPointer() -> rte_timer_cb_t
	{
		Some(Self::callFromC)
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn asFunctionArgument(&mut self) -> *mut c_void
	{
		self as *mut _ as *mut c_void
	}
}
