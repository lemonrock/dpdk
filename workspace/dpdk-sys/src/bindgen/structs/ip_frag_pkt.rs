// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct ip_frag_pkt
{
	pub lru: ip_frag_pkt__bindgen_ty_1,
	pub key: ip_frag_key,
	pub start: u64,
	pub total_size: u32,
	pub frag_size: u32,
	pub last_idx: u32,
	pub frags: [ip_frag; 4usize],
	pub __bindgen_padding_0: [u64; 6usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for ip_frag_pkt
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ip_frag_pkt
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ip_frag_pkt {{ key: {:?} }}", self.key)
	}
}
