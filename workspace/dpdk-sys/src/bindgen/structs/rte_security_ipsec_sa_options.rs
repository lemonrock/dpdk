// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(4))]
pub struct rte_security_ipsec_sa_options
{
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 3usize],
	pub __bindgen_align: [u32; 0usize],
}

impl Default for rte_security_ipsec_sa_options
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_security_ipsec_sa_options
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_security_ipsec_sa_options {{ esn : {:?}, udp_encap : {:?}, copy_dscp : {:?}, copy_flabel : {:?}, copy_df : {:?}, dec_ttl : {:?} }}", self.esn(), self.udp_encap(), self.copy_dscp(), self.copy_flabel(), self.copy_df(), self.dec_ttl())
	}
}

impl rte_security_ipsec_sa_options
{
	
	#[inline(always)]
	pub fn esn(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_esn(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn udp_encap(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_udp_encap(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn copy_dscp(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_copy_dscp(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn copy_flabel(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_copy_flabel(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(3usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn copy_df(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_copy_df(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(4usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn dec_ttl(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_dec_ttl(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(5usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(esn: u32, udp_encap: u32, copy_dscp: u32, copy_flabel: u32, copy_df: u32, dec_ttl: u32) -> __BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let esn: u32 = unsafe { transmute(esn) };
			esn as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let udp_encap: u32 = unsafe { transmute(udp_encap) };
			udp_encap as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let copy_dscp: u32 = unsafe { transmute(copy_dscp) };
			copy_dscp as u64
		});
		__bindgen_bitfield_unit.set(3usize, 1u8, {
			let copy_flabel: u32 = unsafe { transmute(copy_flabel) };
			copy_flabel as u64
		});
		__bindgen_bitfield_unit.set(4usize, 1u8, {
			let copy_df: u32 = unsafe { transmute(copy_df) };
			copy_df as u64
		});
		__bindgen_bitfield_unit.set(5usize, 1u8, {
			let dec_ttl: u32 = unsafe { transmute(dec_ttl) };
			dec_ttl as u64
		});
		__bindgen_bitfield_unit
	}
}
