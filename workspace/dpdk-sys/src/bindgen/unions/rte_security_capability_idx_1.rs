// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_security_capability_idx_1
{
	pub ipsec: rte_security_capability_idx_1_1,
	_bindgen_union_align: [u32; 3usize],
}

impl Default for rte_security_capability_idx_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_security_capability_idx_1
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_security_capability_idx_1 {{ union }}")
	}
}
