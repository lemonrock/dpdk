// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_sched_port_params
{
	pub name: *const c_char,
	pub socket: c_int,
	pub rate: u32,
	pub mtu: u32,
	pub frame_overhead: u32,
	pub n_subports_per_port: u32,
	pub n_pipes_per_subport: u32,
	pub qsize: [u16; 4usize],
	pub pipe_profiles: *mut rte_sched_pipe_params,
	pub n_pipe_profiles: u32,
}

impl Default for rte_sched_port_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_sched_port_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_sched_port_params {{ name: {:?}, qsize: {:?}, pipe_profiles: {:?} }}", self.name, self.qsize, self.pipe_profiles)
	}
}
