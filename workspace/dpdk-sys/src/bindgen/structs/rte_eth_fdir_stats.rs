// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_fdir_stats
{
	pub collision: u32,
	pub free: u32,
	pub maxhash: u32,
	pub maxlen: u32,
	pub add: u64,
	pub remove: u64,
	pub f_add: u64,
	pub f_remove: u64,
	pub guarant_cnt: u32,
	pub best_cnt: u32,
}

impl Default for rte_eth_fdir_stats
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_fdir_stats
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_fdir_stats {{  }}")
	}
}
