// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct rte_table_hash_params
{
	pub name: *const c_char,
	pub key_size: u32,
	pub key_offset: u32,
	pub key_mask: *mut u8,
	pub n_keys: u32,
	pub n_buckets: u32,
	pub f_hash: rte_table_hash_op_hash,
	pub seed: u64,
}

impl Default for rte_table_hash_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_table_hash_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_table_hash_params {{ name: {:?}, key_mask: {:?} }}", self.name, self.key_mask)
	}
}
