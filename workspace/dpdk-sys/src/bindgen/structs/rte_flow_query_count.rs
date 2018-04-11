// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_query_count
{
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
	pub hits: u64,
	pub bytes: u64,
}

impl Default for rte_flow_query_count
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_query_count
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_query_count {{ reset : {:?}, hits_set : {:?}, bytes_set : {:?}, reserved : {:?} }}", self.reset(), self.hits_set(), self.bytes_set(), self.reserved())
	}
}

impl rte_flow_query_count
{
	
	#[inline(always)]
	pub fn reset(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_reset(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hits_set(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_hits_set(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn bytes_set(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_bytes_set(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn reserved(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(3usize, 29u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_reserved(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(3usize, 29u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(reset: u32, hits_set: u32, bytes_set: u32, reserved: u32) -> __BindgenBitfieldUnit<[u8; 4usize], u32>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let reset: u32 = unsafe { transmute(reset) };
			reset as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let hits_set: u32 = unsafe { transmute(hits_set) };
			hits_set as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let bytes_set: u32 = unsafe { transmute(bytes_set) };
			bytes_set as u64
		});
		__bindgen_bitfield_unit.set(3usize, 29u8, {
			let reserved: u32 = unsafe { transmute(reserved) };
			reserved as u64
		});
		__bindgen_bitfield_unit
	}
}
