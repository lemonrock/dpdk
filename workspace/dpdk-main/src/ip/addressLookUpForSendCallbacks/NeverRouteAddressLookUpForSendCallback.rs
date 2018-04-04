// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
pub struct NeverRouteAddressLookUpForSendCallback;

impl AddressLookUpForSendCallback<in_addr> for NeverRouteAddressLookUpForSendCallback
{
	#[inline(always)]
	fn call(&mut self, destinationAddress: *const in_addr, outParameterForResult: *mut tle_dest) -> i32
	{
		NegativeE::EDESTADDRREQ
	}
}

impl AddressLookUpForSendCallback<in6_addr> for NeverRouteAddressLookUpForSendCallback
{
	#[inline(always)]
	fn call(&mut self, destinationAddress: *const in6_addr, outParameterForResult: *mut tle_dest) -> i32
	{
		NegativeE::EDESTADDRREQ
	}
}
