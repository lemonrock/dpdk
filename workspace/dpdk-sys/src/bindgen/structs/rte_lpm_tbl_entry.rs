// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(4))]
pub struct rte_lpm_tbl_entry
{
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
	pub __bindgen_align: [u32; 0usize],
}

impl Default for rte_lpm_tbl_entry
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_lpm_tbl_entry
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_lpm_tbl_entry {{ next_hop : {:?}, valid : {:?}, valid_group : {:?}, depth : {:?} }}", self.next_hop(), self.valid(), self.valid_group(), self.depth())
	}
}

impl rte_lpm_tbl_entry
{
	
	#[inline(always)]
	pub fn next_hop(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 24u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_next_hop(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(0usize, 24u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn valid(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(24usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_valid(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(24usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn valid_group(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(25usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_valid_group(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(25usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn depth(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(26usize, 6u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_depth(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(26usize, 6u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(next_hop: u32, valid: u32, valid_group: u32, depth: u32) -> __BindgenBitfieldUnit<[u8; 4usize], u32>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 24u8, {
			let next_hop: u32 = unsafe { transmute(next_hop) };
			next_hop as u64
		});
		__bindgen_bitfield_unit.set(24usize, 1u8, {
			let valid: u32 = unsafe { transmute(valid) };
			valid as u64
		});
		__bindgen_bitfield_unit.set(25usize, 1u8, {
			let valid_group: u32 = unsafe { transmute(valid_group) };
			valid_group as u64
		});
		__bindgen_bitfield_unit.set(26usize, 6u8, {
			let depth: u32 = unsafe { transmute(depth) };
			depth as u64
		});
		__bindgen_bitfield_unit
	}
}
