// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rust_tle_evq_get(evq: *mut tle_evq, evd: *mut *const c_void, num: u32) -> i32;
	pub fn rust_tle_evq_idle(evq: *mut tle_evq, ev: *mut *mut tle_event, num: u32);
}
