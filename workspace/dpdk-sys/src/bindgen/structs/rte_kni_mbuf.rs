// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_kni_mbuf
{
	pub buf_addr: *mut c_void,
	pub buf_physaddr: u64,
	pub data_off: u16,
	pub pad1: [c_char; 2usize],
	pub nb_segs: u16,
	pub pad4: [c_char; 2usize],
	pub ol_flags: u64,
	pub pad2: [c_char; 4usize],
	pub pkt_len: u32,
	pub data_len: u16,
	pub __bindgen_padding_0: [u8; 22usize],
	pub pad3: [c_char; 8usize],
	pub pool: *mut c_void,
	pub next: *mut c_void,
	pub __bindgen_padding_1: [u64; 5usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_kni_mbuf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_kni_mbuf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_kni_mbuf {{ buf_addr: {:?}, pad1: {:?}, pad4: {:?}, pad2: {:?}, pad3: {:?}, pool: {:?}, next: {:?} }}", self.buf_addr, self.pad1, self.pad4, self.pad2, self.pad3, self.pool, self.next)
	}
}
