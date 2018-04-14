// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(8))]
pub struct rte_mbuf_6_1
{
	pub bitfield_1: BindgenBitfieldUnit<[u8; 8usize], u16>,
	pub __bindgen_align: [u64; 0usize],
}

impl Default for rte_mbuf_6_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mbuf_6_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mbuf_6_1 {{ l2_len : {:?}, l3_len : {:?}, l4_len : {:?}, tso_segsz : {:?}, outer_l3_len : {:?}, outer_l2_len : {:?} }}", self.l2_len(), self.l3_len(), self.l4_len(), self.tso_segsz(), self.outer_l3_len(), self.outer_l2_len())
	}
}

impl rte_mbuf_6_1
{
	
	#[inline(always)]
	pub fn l2_len(&self) -> u64
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 7u8) as u64) }
	}
	
	#[inline(always)]
	pub fn set_l2_len(&mut self, val: u64)
	{
		unsafe {
			let val: u64 = transmute(val);
			self.bitfield_1.set(0usize, 7u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn l3_len(&self) -> u64
	{
		unsafe { transmute(self.bitfield_1.get(7usize, 9u8) as u64) }
	}
	
	#[inline(always)]
	pub fn set_l3_len(&mut self, val: u64)
	{
		unsafe {
			let val: u64 = transmute(val);
			self.bitfield_1.set(7usize, 9u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn l4_len(&self) -> u64
	{
		unsafe { transmute(self.bitfield_1.get(16usize, 8u8) as u64) }
	}
	
	#[inline(always)]
	pub fn set_l4_len(&mut self, val: u64)
	{
		unsafe {
			let val: u64 = transmute(val);
			self.bitfield_1.set(16usize, 8u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn tso_segsz(&self) -> u64
	{
		unsafe { transmute(self.bitfield_1.get(24usize, 16u8) as u64) }
	}
	
	#[inline(always)]
	pub fn set_tso_segsz(&mut self, val: u64)
	{
		unsafe {
			let val: u64 = transmute(val);
			self.bitfield_1.set(24usize, 16u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn outer_l3_len(&self) -> u64
	{
		unsafe { transmute(self.bitfield_1.get(40usize, 9u8) as u64) }
	}
	
	#[inline(always)]
	pub fn set_outer_l3_len(&mut self, val: u64)
	{
		unsafe {
			let val: u64 = transmute(val);
			self.bitfield_1.set(40usize, 9u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn outer_l2_len(&self) -> u64
	{
		unsafe { transmute(self.bitfield_1.get(49usize, 7u8) as u64) }
	}
	
	#[inline(always)]
	pub fn set_outer_l2_len(&mut self, val: u64)
	{
		unsafe {
			let val: u64 = transmute(val);
			self.bitfield_1.set(49usize, 7u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(l2_len: u64, l3_len: u64, l4_len: u64, tso_segsz: u64, outer_l3_len: u64, outer_l2_len: u64) -> BindgenBitfieldUnit<[u8; 8usize], u16>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 8usize], u16> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 7u8, {
			let l2_len: u64 = unsafe { transmute(l2_len) };
			l2_len as u64
		});
		__bindgen_bitfield_unit.set(7usize, 9u8, {
			let l3_len: u64 = unsafe { transmute(l3_len) };
			l3_len as u64
		});
		__bindgen_bitfield_unit.set(16usize, 8u8, {
			let l4_len: u64 = unsafe { transmute(l4_len) };
			l4_len as u64
		});
		__bindgen_bitfield_unit.set(24usize, 16u8, {
			let tso_segsz: u64 = unsafe { transmute(tso_segsz) };
			tso_segsz as u64
		});
		__bindgen_bitfield_unit.set(40usize, 9u8, {
			let outer_l3_len: u64 = unsafe { transmute(outer_l3_len) };
			outer_l3_len as u64
		});
		__bindgen_bitfield_unit.set(49usize, 7u8, {
			let outer_l2_len: u64 = unsafe { transmute(outer_l2_len) };
			outer_l2_len as u64
		});
		__bindgen_bitfield_unit
	}
}
