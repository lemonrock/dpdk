// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_fbk_hash_params
{
	pub name: *const c_char,
	pub entries: u32,
	pub entries_per_bucket: u32,
	pub socket_id: c_int,
	pub hash_func: rte_fbk_hash_fn,
	pub init_val: u32,
}

impl Default for rte_fbk_hash_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_fbk_hash_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_fbk_hash_params {{ name: {:?} }}", self.name)
	}
}
