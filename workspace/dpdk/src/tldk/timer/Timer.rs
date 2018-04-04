// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timer<'a, D>(*mut c_void, &'a TimerWheel<D>) where D: 'a;

impl<'a, D> Timer<'a, D>
{
	#[inline(always)]
	pub fn stop(&mut self)
	{
		unsafe { ::dpdk_sys::tle_timer_stop((self.1).0, self.0) };
	}
}
