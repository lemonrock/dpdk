// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct rte_hash_parameters
{
	pub name: *const c_char,
	pub entries: u32,
	pub reserved: u32,
	pub key_len: u32,
	pub hash_func: rte_hash_function,
	pub hash_func_init_val: u32,
	pub socket_id: c_int,
	pub extra_flag: u8,
}

impl Default for rte_hash_parameters
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_hash_parameters
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_hash_parameters {{ name: {:?} }}", self.name)
	}
}
