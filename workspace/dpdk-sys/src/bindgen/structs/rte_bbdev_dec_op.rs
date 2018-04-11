// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_dec_op
{
	pub status: c_int,
	pub mempool: *mut rte_mempool,
	pub opaque_data: *mut c_void,
	pub turbo_dec: rte_bbdev_op_turbo_dec,
}

impl Default for rte_bbdev_dec_op
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_dec_op
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_dec_op {{ mempool: {:?}, opaque_data: {:?}, turbo_dec: {:?} }}", self.mempool, self.opaque_data, self.turbo_dec)
	}
}
