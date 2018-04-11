// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_member_setsum
{
	pub type_: rte_member_setsum_type,
	pub key_len: u32,
	pub prim_hash_seed: u32,
	pub sec_hash_seed: u32,
	pub bucket_cnt: u32,
	pub bucket_mask: u32,
	pub sig_cmp_fn: rte_member_sig_compare_function,
	pub cache: u8,
	pub num_set: u32,
	pub bits: u32,
	pub bit_mask: u32,
	pub num_hashes: u32,
	pub mul_shift: u32,
	pub div_shift: u32,
	pub table: *mut c_void,
	pub socket_id: u32,
	pub name: [c_char; 32usize],
	pub __bindgen_padding_0: [u32; 7usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_member_setsum
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_member_setsum
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_member_setsum {{ type: {:?}, sig_cmp_fn: {:?}, table: {:?}, name: [{}] }}",
			self.type_,
			self.sig_cmp_fn,
			self.table,
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
				.collect::<String>()
		)
	}
}
