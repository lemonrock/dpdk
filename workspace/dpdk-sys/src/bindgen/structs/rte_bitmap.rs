// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bitmap
{
	pub array1: *mut u64,
	pub array2: *mut u64,
	pub array1_size: u32,
	pub array2_size: u32,
	pub index1: u32,
	pub offset1: u32,
	pub index2: u32,
	pub go2: u32,
	pub memory: __IncompleteArrayField<u8>,
}

impl Default for rte_bitmap
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bitmap
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bitmap {{ array1: {:?}, array2: {:?}, memory: {:?} }}", self.array1, self.array2, self.memory)
	}
}
