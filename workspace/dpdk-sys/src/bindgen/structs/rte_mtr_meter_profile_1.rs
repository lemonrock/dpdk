// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_mtr_meter_profile_1
{
	pub srtcm_rfc2697: BindgenUnionField<rte_mtr_meter_profile_1_1>,
	pub trtcm_rfc2698: BindgenUnionField<rte_mtr_meter_profile_1_2>,
	pub trtcm_rfc4115: BindgenUnionField<rte_mtr_meter_profile_1_3>,
	pub bindgen_union_field: [u64; 4usize],
}

impl Default for rte_mtr_meter_profile_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mtr_meter_profile_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mtr_meter_profile_1 {{ union }}")
	}
}
