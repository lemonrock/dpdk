// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rust_tle_event_active(ev: *mut tle_event, st: tle_ev_state);
	pub fn rust_tle_event_down(ev: *mut tle_event);
	pub fn rust_tle_event_idle(ev: *mut tle_event);
	pub fn rust_tle_event_raise(ev: *mut tle_event);
	pub fn rust_tle_event_state(ev: *const tle_event) -> tle_ev_state;
}
