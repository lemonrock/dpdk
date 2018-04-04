// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Stream
{
	const Protocol: Layer4Protocol;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(opaqueFfiHandle: *mut tle_stream) -> Self;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _opaqueFfiHandle(&mut self) -> *mut tle_stream;
	
	#[inline(always)]
	fn close(&mut self);
}
