// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_timer
{
	pub expire: u64,
	pub sl_next: [*mut rte_timer; 10usize],
	pub status: rte_timer_status,
	pub period: u64,
	pub f: rte_timer_cb_t,
	pub arg: *mut c_void,
}

impl Default for rte_timer
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_timer
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_timer {{ sl_next: {:?}, status: {:?}, arg: {:?} }}", self.sl_next, self.status, self.arg)
	}
}
