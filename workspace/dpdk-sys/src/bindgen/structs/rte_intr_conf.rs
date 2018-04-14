// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(4))]
pub struct rte_intr_conf
{
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 3usize],
	pub __bindgen_align: [u32; 0usize],
}

impl Default for rte_intr_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_intr_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_intr_conf {{ lsc : {:?}, rxq : {:?}, rmv : {:?} }}", self.lsc(), self.rxq(), self.rmv())
	}
}

impl rte_intr_conf
{
	
	#[inline(always)]
	pub fn lsc(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_lsc(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn rxq(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_rxq(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn rmv(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(2usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_rmv(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(lsc: u32, rxq: u32, rmv: u32) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let lsc: u32 = unsafe { transmute(lsc) };
			lsc as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let rxq: u32 = unsafe { transmute(rxq) };
			rxq as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let rmv: u32 = unsafe { transmute(rmv) };
			rmv as u64
		});
		__bindgen_bitfield_unit
	}
}
