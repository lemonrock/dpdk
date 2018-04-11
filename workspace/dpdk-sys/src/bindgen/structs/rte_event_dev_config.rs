// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_event_dev_config
{
	pub dequeue_timeout_ns: u32,
	pub nb_events_limit: i32,
	pub nb_event_queues: u8,
	pub nb_event_ports: u8,
	pub nb_event_queue_flows: u32,
	pub nb_event_port_dequeue_depth: u32,
	pub nb_event_port_enqueue_depth: u32,
	pub event_dev_cfg: u32,
}

impl Default for rte_event_dev_config
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_dev_config
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_dev_config {{  }}")
	}
}
