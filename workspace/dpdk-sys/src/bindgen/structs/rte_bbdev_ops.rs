// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_bbdev_ops
{
	pub setup_queues: rte_bbdev_setup_queues_t,
	pub intr_enable: rte_bbdev_intr_enable_t,
	pub start: rte_bbdev_start_t,
	pub stop: rte_bbdev_stop_t,
	pub close: rte_bbdev_close_t,
	pub info_get: rte_bbdev_info_get_t,
	pub stats_get: rte_bbdev_stats_get_t,
	pub stats_reset: rte_bbdev_stats_reset_t,
	pub queue_setup: rte_bbdev_queue_setup_t,
	pub queue_release: rte_bbdev_queue_release_t,
	pub queue_start: rte_bbdev_queue_start_t,
	pub queue_stop: rte_bbdev_queue_stop_t,
	pub queue_intr_enable: rte_bbdev_queue_intr_enable_t,
	pub queue_intr_disable: rte_bbdev_queue_intr_disable_t,
}

impl Default for rte_bbdev_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_ops {{  }}")
	}
}
