// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_driver_info
{
	pub driver_name: *const c_char,
	pub max_num_queues: c_uint,
	pub queue_size_lim: u32,
	pub hardware_accelerated: bool,
	pub max_dl_queue_priority: u8,
	pub max_ul_queue_priority: u8,
	pub queue_intr_supported: bool,
	pub min_alignment: u16,
	pub default_queue_conf: rte_bbdev_queue_conf,
	pub capabilities: *const rte_bbdev_op_cap,
	pub cpu_flag_reqs: *const rte_cpu_flag_t,
}

impl Default for rte_bbdev_driver_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_driver_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_driver_info {{ driver_name: {:?}, default_queue_conf: {:?}, capabilities: {:?}, cpu_flag_reqs: {:?} }}", self.driver_name, self.default_queue_conf, self.capabilities, self.cpu_flag_reqs)
	}
}
