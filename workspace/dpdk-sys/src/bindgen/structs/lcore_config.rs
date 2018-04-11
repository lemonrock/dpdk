// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct lcore_config
{
	pub detected: c_uint,
	pub thread_id: pthread_t,
	pub pipe_master2slave: [c_int; 2usize],
	pub pipe_slave2master: [c_int; 2usize],
	pub f: lcore_function_t,
	pub arg: *mut c_void,
	pub ret: c_int,
	pub state: rte_lcore_state_t,
	pub socket_id: c_uint,
	pub core_id: c_uint,
	pub core_index: c_int,
	pub cpuset: rte_cpuset_t,
	pub core_role: u8,
}

impl Default for lcore_config
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for lcore_config
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "lcore_config {{ pipe_master2slave: {:?}, pipe_slave2master: {:?}, f: {:?}, arg: {:?}, state: {:?} }}", self.pipe_master2slave, self.pipe_slave2master, self.f, self.arg, self.state)
	}
}
