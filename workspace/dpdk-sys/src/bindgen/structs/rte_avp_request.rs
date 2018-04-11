// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct rte_avp_request
{
	pub req_id: u32,
	pub __bindgen_anon_1: rte_avp_request__bindgen_ty_1,
	pub result: i32,
}

impl Default for rte_avp_request
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

