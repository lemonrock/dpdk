// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct rte_memzone
{
	pub name: [c_char; 32usize],
	pub _1: rte_memzone_1,
	pub _2: rte_memzone_2,
	pub len: usize,
	pub hugepage_sz: u64,
	pub socket_id: i32,
	pub flags: u32,
	pub memseg_id: u32,
}

impl Default for rte_memzone
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

