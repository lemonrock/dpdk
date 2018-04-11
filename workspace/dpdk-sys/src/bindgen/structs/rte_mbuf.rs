// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_mbuf
{
	pub cacheline0: MARKER,
	pub buf_addr: *mut c_void,
	pub __bindgen_anon_1: rte_mbuf__bindgen_ty_1,
	pub rearm_data: MARKER64,
	pub data_off: u16,
	pub __bindgen_anon_2: rte_mbuf__bindgen_ty_2,
	pub nb_segs: u16,
	pub port: u16,
	pub ol_flags: u64,
	pub rx_descriptor_fields1: MARKER,
	pub __bindgen_anon_3: rte_mbuf__bindgen_ty_3,
	pub pkt_len: u32,
	pub data_len: u16,
	pub vlan_tci: u16,
	pub hash: rte_mbuf__bindgen_ty_4,
	pub vlan_tci_outer: u16,
	pub buf_len: u16,
	pub timestamp: u64,
	pub cacheline1: MARKER,
	pub __bindgen_anon_4: rte_mbuf__bindgen_ty_5,
	pub pool: *mut rte_mempool,
	pub next: *mut rte_mbuf,
	pub __bindgen_anon_5: rte_mbuf__bindgen_ty_6,
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
		write!(f, "rte_mbuf {{ buf_addr: {:?}, __bindgen_anon_1: {:?}, __bindgen_anon_2: {:?}, __bindgen_anon_3: {:?}, hash: {:?}, __bindgen_anon_4: {:?}, pool: {:?}, next: {:?}, __bindgen_anon_5: {:?} }}", self.buf_addr, self.__bindgen_anon_1, self.__bindgen_anon_2, self.__bindgen_anon_3, self.hash, self.__bindgen_anon_4, self.pool, self.next, self.__bindgen_anon_5)
	}
}
