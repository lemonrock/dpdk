// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_op_dec_tb_params
{
	pub k_neg: u16,
	pub k_pos: u16,
	pub c_neg: u8,
	pub c: u8,
	pub cab: u8,
	pub ea: u32,
	pub eb: u32,
}

impl Default for rte_bbdev_op_dec_tb_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_op_dec_tb_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_op_dec_tb_params {{  }}")
	}
}
