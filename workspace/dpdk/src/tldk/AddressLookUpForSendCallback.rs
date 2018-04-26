// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait AddressLookUpForSendCallback<IpAddress: Sized> : Sized
{
	/// Should return 0 or a value from NegativeE::
	#[inline(always)]
	fn call(&mut self, destinationAddress: *const IpAddress, outParameterForResult: *mut tle_dest) -> i32;
	
	#[doc(hidden)]
	#[inline(always)]
	unsafe extern "C" fn callFromC(opaque: *mut c_void, addr: *const IpAddress, res: *mut tle_dest) -> c_int
	{
		let us: &mut Self = &mut *(opaque as *mut Self);
		
		let result = catch_unwind(AssertUnwindSafe(||
		{
			debug_assert!(opaque.is_not_null(), "opaque was null");
			debug_assert!(addr.is_not_null(), "addr was null");
			debug_assert!(res.is_not_null(), "tle_dest was null");
			
			let resultCode = us.call(addr, res);
			debug_assert!(resultCode <= 0, "Result was a positive value, '{}'", resultCode);
			
			resultCode
		}));
		
		let resultCode = match result
		{
			Ok(resultCode) => resultCode,
			Err(error) =>
			{
				// We do this because we can not panic across FFI boundaries
				stderrln!("Panic in AddressLookUpForSendCallback, returning ENOSYS instead of '{:?}'", error);
				NegativeE::ENOSYS
			},
		};

		forget(us);
		
		resultCode
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn asFunctionPointer() -> Option<unsafe extern "C" fn(opaque: *mut c_void, addr: *const IpAddress, res: *mut tle_dest) -> c_int>
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
