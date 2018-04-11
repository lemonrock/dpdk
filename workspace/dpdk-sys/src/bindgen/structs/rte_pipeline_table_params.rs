// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_pipeline_table_params
{
	pub ops: *mut rte_table_ops,
	pub arg_create: *mut c_void,
	pub f_action_hit: rte_pipeline_table_action_handler_hit,
	pub f_action_miss: rte_pipeline_table_action_handler_miss,
	pub arg_ah: *mut c_void,
	pub action_data_size: u32,
}

impl Default for rte_pipeline_table_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_pipeline_table_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_pipeline_table_params {{ ops: {:?}, arg_create: {:?}, arg_ah: {:?} }}", self.ops, self.arg_create, self.arg_ah)
	}
}
