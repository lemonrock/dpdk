// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(8))]
pub struct rte_eth_link
{
	pub link_speed: u32,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 3usize],
	pub __bindgen_align: [u64; 0usize],
}

impl Default for rte_eth_link
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_link
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_link {{ link_duplex : {:?}, link_autoneg : {:?}, link_status : {:?} }}", self.link_duplex(), self.link_autoneg(), self.link_status())
	}
}

impl rte_eth_link
{
	
	#[inline(always)]
	pub fn link_duplex(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_link_duplex(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn link_autoneg(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_link_autoneg(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn link_status(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(2usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_link_status(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(link_duplex: u16, link_autoneg: u16, link_status: u16) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let link_duplex: u16 = unsafe { transmute(link_duplex) };
			link_duplex as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let link_autoneg: u16 = unsafe { transmute(link_autoneg) };
			link_autoneg as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let link_status: u16 = unsafe { transmute(link_status) };
			link_status as u64
		});
		__bindgen_bitfield_unit
	}
}
