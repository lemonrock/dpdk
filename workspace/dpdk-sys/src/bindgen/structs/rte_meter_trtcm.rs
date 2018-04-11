// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_meter_trtcm
{
	pub time_tc: u64,
	pub time_tp: u64,
	pub tc: u64,
	pub tp: u64,
	pub cbs: u64,
	pub pbs: u64,
	pub cir_period: u64,
	pub cir_bytes_per_period: u64,
	pub pir_period: u64,
	pub pir_bytes_per_period: u64,
}

impl Default for rte_meter_trtcm
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_meter_trtcm
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_meter_trtcm {{  }}")
	}
}
