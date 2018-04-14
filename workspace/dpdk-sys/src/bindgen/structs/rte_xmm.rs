// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_xmm
{
	pub x: BindgenUnionField<xmm_t>,
	pub u8: BindgenUnionField<[u8; 16usize]>,
	pub u16: BindgenUnionField<[u16; 8usize]>,
	pub u32: BindgenUnionField<[u32; 4usize]>,
	pub u64: BindgenUnionField<[u64; 2usize]>,
	pub pd: BindgenUnionField<[f64; 2usize]>,
	pub bindgen_union_field: [u8; 16usize],
}

impl Default for rte_xmm
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_xmm
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_xmm {{ union }}")
	}
}
