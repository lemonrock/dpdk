// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_pipeline_table_entry
{
	pub action: rte_pipeline_action,
	pub __bindgen_anon_1: rte_pipeline_table_entry__bindgen_ty_1,
	pub action_data: __IncompleteArrayField<u8>,
}

impl Default for rte_pipeline_table_entry
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_pipeline_table_entry
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_pipeline_table_entry {{ action: {:?}, __bindgen_anon_1: {:?}, action_data: {:?} }}", self.action, self.__bindgen_anon_1, self.action_data)
	}
}
