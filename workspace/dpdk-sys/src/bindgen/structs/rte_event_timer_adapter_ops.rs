// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_event_timer_adapter_ops
{
	pub init: rte_event_timer_adapter_init_t,
	pub uninit: rte_event_timer_adapter_uninit_t,
	pub start: rte_event_timer_adapter_start_t,
	pub stop: rte_event_timer_adapter_stop_t,
	pub get_info: rte_event_timer_adapter_get_info_t,
	pub stats_get: rte_event_timer_adapter_stats_get_t,
	pub stats_reset: rte_event_timer_adapter_stats_reset_t,
	pub arm_burst: rte_event_timer_arm_burst_t,
	pub arm_tmo_tick_burst: rte_event_timer_arm_tmo_tick_burst_t,
	pub cancel_burst: rte_event_timer_cancel_burst_t,
}

impl Default for rte_event_timer_adapter_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_timer_adapter_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_timer_adapter_ops {{  }}")
	}
}
