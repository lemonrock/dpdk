// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_event_dev_info
{
	pub driver_name: *const c_char,
	pub dev: *mut rte_device,
	pub min_dequeue_timeout_ns: u32,
	pub max_dequeue_timeout_ns: u32,
	pub dequeue_timeout_ns: u32,
	pub max_event_queues: u8,
	pub max_event_queue_flows: u32,
	pub max_event_queue_priority_levels: u8,
	pub max_event_priority_levels: u8,
	pub max_event_ports: u8,
	pub max_event_port_dequeue_depth: u8,
	pub max_event_port_enqueue_depth: u32,
	pub max_num_events: i32,
	pub event_dev_cap: u32,
}

impl Default for rte_event_dev_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_dev_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_dev_info {{ driver_name: {:?}, dev: {:?} }}", self.driver_name, self.dev)
	}
}
