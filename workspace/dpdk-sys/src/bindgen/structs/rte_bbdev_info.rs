// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_info
{
	pub socket_id: c_int,
	pub dev_name: *const c_char,
	pub bus: *const rte_bus,
	pub num_queues: u16,
	pub started: bool,
	pub drv: rte_bbdev_driver_info,
}

impl Default for rte_bbdev_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_info {{ dev_name: {:?}, bus: {:?}, drv: {:?} }}", self.dev_name, self.bus, self.drv)
	}
}
