// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_op_turbo_enc
{
	pub input: rte_bbdev_op_data,
	pub output: rte_bbdev_op_data,
	pub op_flags: u32,
	pub rv_index: u8,
	pub code_block_mode: u8,
	pub __bindgen_anon_1: rte_bbdev_op_turbo_enc__bindgen_ty_1,
}

impl Default for rte_bbdev_op_turbo_enc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_op_turbo_enc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_op_turbo_enc {{ input: {:?}, output: {:?}, __bindgen_anon_1: {:?} }}", self.input, self.output, self.__bindgen_anon_1)
	}
}
