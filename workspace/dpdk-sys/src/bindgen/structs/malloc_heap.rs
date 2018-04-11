// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct malloc_heap
{
	pub lock: rte_spinlock_t,
	pub free_head: [malloc_heap__bindgen_ty_1; 13usize],
	pub alloc_count: c_uint,
	pub total_size: usize,
	pub __bindgen_align: [u8; 0usize],
}

impl Default for malloc_heap
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for malloc_heap
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "malloc_heap {{ lock: {:?} }}", self.lock)
	}
}
