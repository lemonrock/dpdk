// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_gso_ctx
{
	pub direct_pool: *mut rte_mempool,
	pub indirect_pool: *mut rte_mempool,
	pub flag: u64,
	pub gso_types: u32,
	pub gso_size: u16,
}

impl Default for rte_gso_ctx
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_gso_ctx
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_gso_ctx {{ direct_pool: {:?}, indirect_pool: {:?} }}", self.direct_pool, self.indirect_pool)
	}
}
