// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct ip_frag_tbl_stat
{
	pub find_num: u64,
	pub add_num: u64,
	pub del_num: u64,
	pub reuse_num: u64,
	pub fail_total: u64,
	pub fail_nospace: u64,
	pub __bindgen_padding_0: [u64; 2usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for ip_frag_tbl_stat
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ip_frag_tbl_stat
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ip_frag_tbl_stat {{  }}")
	}
}
