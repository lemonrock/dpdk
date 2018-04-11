// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_queue_data
{
	pub queue_private: *mut c_void,
	pub conf: rte_bbdev_queue_conf,
	pub queue_stats: rte_bbdev_stats,
	pub started: bool,
}

impl Default for rte_bbdev_queue_data
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_queue_data
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_queue_data {{ queue_private: {:?}, conf: {:?}, queue_stats: {:?} }}", self.queue_private, self.conf, self.queue_stats)
	}
}
