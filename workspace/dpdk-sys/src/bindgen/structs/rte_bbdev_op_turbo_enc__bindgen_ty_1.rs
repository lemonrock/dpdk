// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_op_turbo_enc__bindgen_ty_1
{
	pub cb_params: __BindgenUnionField<rte_bbdev_op_enc_cb_params>,
	pub tb_params: __BindgenUnionField<rte_bbdev_op_enc_tb_params>,
	pub bindgen_union_field: [u32; 6usize],
}

impl Default for rte_bbdev_op_turbo_enc__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_op_turbo_enc__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_op_turbo_enc__bindgen_ty_1 {{ union }}")
	}
}
