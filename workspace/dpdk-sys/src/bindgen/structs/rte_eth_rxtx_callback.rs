// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_rxtx_callback
{
	pub next: *mut rte_eth_rxtx_callback,
	pub fn_: rte_eth_rxtx_callback_1,
	pub param: *mut c_void,
}

impl Default for rte_eth_rxtx_callback
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_rxtx_callback
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_rxtx_callback {{ next: {:?}, fn: {:?}, param: {:?} }}", self.next, self.fn_, self.param)
	}
}
