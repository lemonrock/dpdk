// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_mbuf
{
	pub cacheline0: MARKER,
	pub buf_addr: *mut c_void,
	pub _1: rte_mbuf_1,
	pub rearm_data: MARKER64,
	pub data_off: u16,
	pub _2: rte_mbuf_2,
	pub nb_segs: u16,
	pub port: u16,
	pub ol_flags: u64,
	pub rx_descriptor_fields1: MARKER,
	pub _3: rte_mbuf_3,
	pub pkt_len: u32,
	pub data_len: u16,
	pub vlan_tci: u16,
	pub hash: rte_mbuf_4,
	pub vlan_tci_outer: u16,
	pub buf_len: u16,
	pub timestamp: u64,
	pub cacheline1: MARKER,
	pub _4: rte_mbuf_5,
	pub pool: *mut rte_mempool,
	pub next: *mut rte_mbuf,
	pub _5: rte_mbuf_6,
	pub priv_size: u16,
	pub timesync: u16,
	pub seqn: u32,
	pub __bindgen_padding_0: [u64; 3usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_mbuf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mbuf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mbuf {{ buf_addr: {:?}, _1: {:?}, _2: {:?}, _3: {:?}, hash: {:?}, _4: {:?}, pool: {:?}, next: {:?}, _5: {:?} }}", self.buf_addr, self._1, self._2, self._3, self.hash, self._4, self.pool, self.next, self._5)
	}
}
