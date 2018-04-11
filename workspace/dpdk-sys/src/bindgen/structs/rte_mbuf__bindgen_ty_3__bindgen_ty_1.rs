// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(4))]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1
{
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
	pub __bindgen_anon_1: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
	pub _bitfield_2: __BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_align: [u32; 0usize],
}

impl Default for rte_mbuf__bindgen_ty_3__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mbuf__bindgen_ty_3__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mbuf__bindgen_ty_3__bindgen_ty_1 {{ l2_type : {:?}, l3_type : {:?}, l4_type : {:?}, tun_type : {:?}, __bindgen_anon_1: {:?}, inner_l4_type : {:?} }}", self.l2_type(), self.l3_type(), self.l4_type(), self.tun_type(), self.__bindgen_anon_1, self.inner_l4_type())
	}
}

impl rte_mbuf__bindgen_ty_3__bindgen_ty_1
{
	
	#[inline(always)]
	pub fn l2_type(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_l2_type(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(0usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn l3_type(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_l3_type(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(4usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn l4_type(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(8usize, 4u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_l4_type(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(8usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn tun_type(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(12usize, 4u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_tun_type(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(12usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(l2_type: u32, l3_type: u32, l4_type: u32, tun_type: u32) -> __BindgenBitfieldUnit<[u8; 2usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 4u8, {
			let l2_type: u32 = unsafe { transmute(l2_type) };
			l2_type as u64
		});
		__bindgen_bitfield_unit.set(4usize, 4u8, {
			let l3_type: u32 = unsafe { transmute(l3_type) };
			l3_type as u64
		});
		__bindgen_bitfield_unit.set(8usize, 4u8, {
			let l4_type: u32 = unsafe { transmute(l4_type) };
			l4_type as u64
		});
		__bindgen_bitfield_unit.set(12usize, 4u8, {
			let tun_type: u32 = unsafe { transmute(tun_type) };
			tun_type as u64
		});
		__bindgen_bitfield_unit
	}
	
	#[inline(always)]
	pub fn inner_l4_type(&self) -> u32
	{
		unsafe { transmute(self._bitfield_2.get(0usize, 4u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_inner_l4_type(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_2.set(0usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_2(inner_l4_type: u32) -> __BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 4u8, {
			let inner_l4_type: u32 = unsafe { transmute(inner_l4_type) };
			inner_l4_type as u64
		});
		__bindgen_bitfield_unit
	}
}
