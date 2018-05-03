// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_event_timer_adapter_data
{
	pub id: u8,
	pub event_dev_id: u8,
	pub socket_id: u32,
	pub event_port_id: u8,
	pub mz: *const rte_memzone,
	pub conf: rte_event_timer_adapter_conf,
	pub caps: u32,
	pub adapter_priv: *mut c_void,
	pub service_inited: u8,
	pub service_id: u32,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 31usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_event_timer_adapter_data
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_timer_adapter_data
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_timer_adapter_data {{ mz: {:?}, conf: {:?}, adapter_priv: {:?}, started : {:?} }}", self.mz, self.conf, self.adapter_priv, self.started())
	}
}

impl rte_event_timer_adapter_data
{
	
	#[inline(always)]
	pub fn started(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_started(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(started: u8) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let started: u8 = unsafe { transmute(started) };
			started as u64
		});
		__bindgen_bitfield_unit
	}
}
