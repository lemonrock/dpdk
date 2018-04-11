// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_eth_hash_global_conf
{
	pub hash_func: rte_eth_hash_function,
	pub sym_hash_enable_mask: [u64; 1usize],
	pub valid_bit_mask: [u64; 1usize],
}

impl Default for rte_eth_hash_global_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_hash_global_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_hash_global_conf {{ hash_func: {:?}, sym_hash_enable_mask: {:?}, valid_bit_mask: {:?} }}", self.hash_func, self.sym_hash_enable_mask, self.valid_bit_mask)
	}
}
