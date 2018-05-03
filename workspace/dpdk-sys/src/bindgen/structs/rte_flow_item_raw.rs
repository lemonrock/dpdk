// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_item_raw
{
	pub bitfield_1: BindgenBitfieldUnit<[u8; 4usize], u32>,
	pub offset: i32,
	pub limit: u16,
	pub length: u16,
	pub pattern: *const u8,
}

impl Default for rte_flow_item_raw
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_item_raw
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_item_raw {{ relative : {:?}, search : {:?}, reserved : {:?}, pattern: {:?} }}", self.relative(), self.search(), self.reserved(), self.pattern)
	}
}

impl rte_flow_item_raw
{
	
	#[inline(always)]
	pub fn relative(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_relative(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn search(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_search(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn reserved(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(2usize, 30u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_reserved(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(2usize, 30u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(relative: u32, search: u32, reserved: u32) -> BindgenBitfieldUnit<[u8; 4usize], u32>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 4usize], u32> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let relative: u32 = unsafe { transmute(relative) };
			relative as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let search: u32 = unsafe { transmute(search) };
			search as u64
		});
		__bindgen_bitfield_unit.set(2usize, 30u8, {
			let reserved: u32 = unsafe { transmute(reserved) };
			reserved as u64
		});
		__bindgen_bitfield_unit
	}
}
