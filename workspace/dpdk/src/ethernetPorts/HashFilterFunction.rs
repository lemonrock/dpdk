// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HashFilterFunction
{
	Toeplitz = 1,
	SimpleExclusiveOr = 2,
}

impl HashFilterFunction
{
	#[inline(always)]
	pub fn as_rte_eth_hash_function(&self) -> rte_eth_hash_function
	{
		unsafe { transmute(*self as u32) }
	}
}
