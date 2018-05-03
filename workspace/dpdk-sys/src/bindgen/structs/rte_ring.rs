// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_ring
{
	pub name: [c_char; 32usize],
	pub flags: c_int,
	pub memzone: *const rte_memzone,
	pub size: u32,
	pub mask: u32,
	pub capacity: u32,
	pub __bindgen_padding_0: [u8; 4usize],
	pub pad0: c_char,
	pub __bindgen_padding_1: [u32; 15usize],
	pub prod: rte_ring_headtail,
	pub __bindgen_padding_2: [u8; 52usize],
	pub pad1: c_char,
	pub __bindgen_padding_3: [u32; 15usize],
	pub cons: rte_ring_headtail,
	pub __bindgen_padding_4: [u8; 52usize],
	pub pad2: c_char,
	pub __bindgen_padding_5: [u8; 63usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_ring
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_ring
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_ring {{ name: [{}], memzone: {:?}, prod: {:?}, cons: {:?} }}",
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.memzone,
			self.prod,
			self.cons
		)
	}
}
