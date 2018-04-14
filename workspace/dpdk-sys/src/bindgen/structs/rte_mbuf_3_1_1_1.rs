// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct rte_mbuf_3_1_1_1
{
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
}

impl Default for rte_mbuf_3_1_1_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mbuf_3_1_1_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mbuf_3_1_1_1 {{ inner_l2_type : {:?}, inner_l3_type : {:?} }}", self.inner_l2_type(), self.inner_l3_type())
	}
}

impl rte_mbuf_3_1_1_1
{
	
	#[inline(always)]
	pub fn inner_l2_type(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 4u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_inner_l2_type(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(0usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn inner_l3_type(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(4usize, 4u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_inner_l3_type(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(4usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(inner_l2_type: u8, inner_l3_type: u8) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 4u8, {
			let inner_l2_type: u8 = unsafe { transmute(inner_l2_type) };
			inner_l2_type as u64
		});
		__bindgen_bitfield_unit.set(4usize, 4u8, {
			let inner_l3_type: u8 = unsafe { transmute(inner_l3_type) };
			inner_l3_type as u64
		});
		__bindgen_bitfield_unit
	}
}
