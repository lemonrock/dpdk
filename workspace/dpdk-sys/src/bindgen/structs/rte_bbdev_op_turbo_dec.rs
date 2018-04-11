// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_op_turbo_dec
{
	pub input: rte_bbdev_op_data,
	pub hard_output: rte_bbdev_op_data,
	pub soft_output: rte_bbdev_op_data,
	pub op_flags: u32,
	pub rv_index: u8,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub iter_count: u8,
	pub ext_scale: u8,
	pub num_maps: u8,
	pub code_block_mode: u8,
	pub __bindgen_anon_1: rte_bbdev_op_turbo_dec__bindgen_ty_1,
}

impl Default for rte_bbdev_op_turbo_dec
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_op_turbo_dec
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev_op_turbo_dec {{ input: {:?}, hard_output: {:?}, soft_output: {:?}, iter_min : {:?}, iter_max : {:?}, __bindgen_anon_1: {:?} }}", self.input, self.hard_output, self.soft_output, self.iter_min(), self.iter_max(), self.__bindgen_anon_1)
	}
}

impl rte_bbdev_op_turbo_dec
{
	
	#[inline(always)]
	pub fn iter_min(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_iter_min(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(0usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn iter_max(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_iter_max(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(4usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(iter_min: u8, iter_max: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 4u8, {
			let iter_min: u8 = unsafe { transmute(iter_min) };
			iter_min as u64
		});
		__bindgen_bitfield_unit.set(4usize, 4u8, {
			let iter_max: u8 = unsafe { transmute(iter_max) };
			iter_max as u64
		});
		__bindgen_bitfield_unit
	}
}
