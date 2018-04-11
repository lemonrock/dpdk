// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_jobstats_context
{
	pub state_time: u64,
	pub loop_executed_jobs: u64,
	pub exec_time: u64,
	pub min_exec_time: u64,
	pub max_exec_time: u64,
	pub management_time: u64,
	pub min_management_time: u64,
	pub max_management_time: u64,
	pub start_time: u64,
	pub job_exec_cnt: u64,
	pub loop_cnt: u64,
	pub __bindgen_padding_0: [u64; 5usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_jobstats_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_jobstats_context
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_jobstats_context {{  }}")
	}
}
