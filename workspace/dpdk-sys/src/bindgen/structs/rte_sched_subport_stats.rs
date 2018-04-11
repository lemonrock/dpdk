// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_sched_subport_stats
{
	pub n_pkts_tc: [u32; 4usize],
	pub n_pkts_tc_dropped: [u32; 4usize],
	pub n_bytes_tc: [u32; 4usize],
	pub n_bytes_tc_dropped: [u32; 4usize],
}

impl Default for rte_sched_subport_stats
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_sched_subport_stats
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_sched_subport_stats {{ n_pkts_tc: {:?}, n_pkts_tc_dropped: {:?}, n_bytes_tc: {:?}, n_bytes_tc_dropped: {:?} }}", self.n_pkts_tc, self.n_pkts_tc_dropped, self.n_bytes_tc, self.n_bytes_tc_dropped)
	}
}
