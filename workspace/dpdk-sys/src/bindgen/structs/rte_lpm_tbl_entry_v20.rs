// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_lpm_tbl_entry_v20
{
	pub _1: rte_lpm_tbl_entry_v20_1,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
}

impl Default for rte_lpm_tbl_entry_v20
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_lpm_tbl_entry_v20
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_lpm_tbl_entry_v20 {{ _1: {:?}, valid : {:?}, valid_group : {:?}, depth : {:?} }}", self._1, self.valid(), self.valid_group(), self.depth())
	}
}

impl rte_lpm_tbl_entry_v20
{
	
	#[inline(always)]
	pub fn valid(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_valid(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn valid_group(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_valid_group(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn depth(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(2usize, 6u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_depth(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(2usize, 6u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(valid: u8, valid_group: u8, depth: u8) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let valid: u8 = unsafe { transmute(valid) };
			valid as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let valid_group: u8 = unsafe { transmute(valid_group) };
			valid_group as u64
		});
		__bindgen_bitfield_unit.set(2usize, 6u8, {
			let depth: u8 = unsafe { transmute(depth) };
			depth as u64
		});
		__bindgen_bitfield_unit
	}
}
