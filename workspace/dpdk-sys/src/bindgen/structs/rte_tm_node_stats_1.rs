// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_tm_node_stats_1
{
	pub n_pkts_dropped: [u64; 3usize],
	pub n_bytes_dropped: [u64; 3usize],
	pub n_pkts_queued: u64,
	pub n_bytes_queued: u64,
}

impl Default for rte_tm_node_stats_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_tm_node_stats_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_tm_node_stats_1 {{ n_pkts_dropped: {:?}, n_bytes_dropped: {:?} }}", self.n_pkts_dropped, self.n_bytes_dropped)
	}
}
