// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait EdgeTriggeredCallback : Sized
{
	#[inline(always)]
	fn call(&mut self);
	
	#[doc(hidden)]
	#[inline(always)]
	unsafe extern "C" fn callFromC(arg1: *mut c_void, arg2: *mut tle_stream)
	{
		let us: &mut Self = &mut *(arg1 as *mut Self);
		
		let result = catch_unwind(AssertUnwindSafe(||
		{
			debug_assert!(!arg1.is_null(), "opaque was null");
			debug_assert!(!arg2.is_null(), "addr was null");
			
			us.call()
		}));
		
		if let Err(error) = result
		{
			// We do this because we can not panic across FFI boundaries
			stderrln!("Panic in EdgeTriggeredCallback was '{:?}'", error);
		}
		
		forget(us);
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn asFunctionPointer() -> Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *mut tle_stream)>
	{
		Some(Self::callFromC)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _to_tle_stream_cb(this: Box<Self>) -> tle_stream_cb
	{
		tle_stream_cb
		{
			func: Self::asFunctionPointer(),
			data: Box::into_raw(this) as *mut c_void,
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _dropCallback(callback: tle_stream_cb)
	{
		let this = callback.data;
		drop(unsafe { Box::from_raw(this as *mut Self) });
	}
}
