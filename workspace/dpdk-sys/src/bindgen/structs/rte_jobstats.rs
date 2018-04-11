// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_jobstats
{
	pub period: u64,
	pub min_period: u64,
	pub max_period: u64,
	pub target: i64,
	pub update_period_cb: rte_job_update_period_cb_t,
	pub exec_time: u64,
	pub min_exec_time: u64,
	pub max_exec_time: u64,
	pub exec_cnt: u64,
	pub name: [c_char; 32usize],
	pub context: *mut rte_jobstats_context,
	pub __bindgen_padding_0: [u64; 2usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_jobstats
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_jobstats
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_jobstats {{ name: [{}], context: {:?} }}",
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.context
		)
	}
}
