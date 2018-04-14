// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_rawdev
{
	pub socket_id: c_int,
	pub dev_id: u16,
	pub dev_ops: *const rte_rawdev_ops,
	pub device: *mut rte_device,
	pub driver_name: *const c_char,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub dev_private: rte_rawdev_obj_t,
	pub name: [c_char; 64usize],
	pub __bindgen_padding_0: [u64; 2usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_rawdev
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_rawdev
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_rawdev {{ dev_ops: {:?}, device: {:?}, driver_name: {:?}, attached : {:?}, started : {:?}, name: [{}] }}",
			self.dev_ops,
			self.device,
			self.driver_name,
			self.attached(),
			self.started(),
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>()
		)
	}
}

impl rte_rawdev
{
	
	#[inline(always)]
	pub fn attached(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_attached(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn started(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_started(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(attached: u8, started: u8) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let attached: u8 = unsafe { transmute(attached) };
			attached as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let started: u8 = unsafe { transmute(started) };
			started as u64
		});
		__bindgen_bitfield_unit
	}
}
