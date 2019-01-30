// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Caught an unwind.
///
/// Log it to to syslog.
#[inline(always)]
pub fn caught_unwind_and_log_it_to_syslog(panic_payload: &(Any + 'static + Send))
{
	let hyper_thread = to_c_string_robustly(format!("{}", HyperThread::current_hyper_thread().0));

	let cause = to_c_string_robustly(panic_payload_to_cause(panic_payload));

	unsafe { syslog(LOG_ERR, b"HyperThread:%s:Cause:%s\0".as_ptr() as *const _, hyper_thread.as_ptr(), cause.as_ptr()) }
}
