// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(8))]
pub struct rte_avp_fifo
{
	pub write: c_uint,
	pub read: c_uint,
	pub len: c_uint,
	pub elem_size: c_uint,
	pub buffer: __IncompleteArrayField<*mut c_void>,
	pub __bindgen_align: [u64; 0usize],
}

impl Default for rte_avp_fifo
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_avp_fifo
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_avp_fifo {{ buffer: {:?} }}", self.buffer)
	}
}
