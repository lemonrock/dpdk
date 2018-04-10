// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Enable or disable performance events.
#[inline(always)]
pub fn adjust_performance_events(enable_performance_events: bool)
{
	let control_value = if enable_performance_events
	{
		PR_TASK_PERF_EVENTS_ENABLE
	}
	else
	{
		PR_TASK_PERF_EVENTS_DISABLE
	};
	unsafe { prctl(control_value) };
}
