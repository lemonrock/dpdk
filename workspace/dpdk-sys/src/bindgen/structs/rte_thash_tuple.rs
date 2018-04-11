// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, align(16))]
pub struct rte_thash_tuple
{
	pub v4: __BindgenUnionField<rte_ipv4_tuple>,
	pub v6: __BindgenUnionField<rte_ipv6_tuple>,
	pub bindgen_union_field: [u8; 48usize],
}

impl Default for rte_thash_tuple
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_thash_tuple
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_thash_tuple {{ union }}")
	}
}
