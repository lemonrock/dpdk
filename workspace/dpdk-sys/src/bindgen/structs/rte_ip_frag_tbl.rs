// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_ip_frag_tbl
{
	pub max_cycles: u64,
	pub entry_mask: u32,
	pub max_entries: u32,
	pub use_entries: u32,
	pub bucket_entries: u32,
	pub nb_entries: u32,
	pub nb_buckets: u32,
	pub last: *mut ip_frag_pkt,
	pub lru: ip_pkt_list,
	pub __bindgen_padding_0: u64,
	pub stat: ip_frag_tbl_stat,
	pub pkt: __IncompleteArrayField<ip_frag_pkt>,
}

impl Default for rte_ip_frag_tbl
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_ip_frag_tbl
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_ip_frag_tbl {{ last: {:?}, lru: {:?}, stat: {:?}, pkt: {:?} }}", self.last, self.lru, self.stat, self.pkt)
	}
}
