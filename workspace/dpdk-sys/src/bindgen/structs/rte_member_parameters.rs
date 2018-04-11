// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_member_parameters
{
	pub name: *const c_char,
	pub type_: rte_member_setsum_type,
	pub is_cache: u8,
	pub num_keys: u32,
	pub key_len: u32,
	pub num_set: u32,
	pub false_positive_rate: f32,
	pub prim_hash_seed: u32,
	pub sec_hash_seed: u32,
	pub socket_id: c_int,
}

impl Default for rte_member_parameters
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_member_parameters
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_member_parameters {{ name: {:?}, type: {:?} }}", self.name, self.type_)
	}
}
