// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(4))]
pub struct rte_event_1_1
{
	pub bitfield_1: BindgenBitfieldUnit<[u8; 8usize], u32>,
	pub queue_id: u8,
	pub priority: u8,
	pub impl_opaque: u8,
	pub __bindgen_align: [u32; 0usize],
}

impl Default for rte_event_1_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_1_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_1_1 {{ flow_id : {:?}, sub_event_type : {:?}, event_type : {:?}, op : {:?}, rsvd : {:?}, sched_type : {:?} }}", self.flow_id(), self.sub_event_type(), self.event_type(), self.op(), self.rsvd(), self.sched_type())
	}
}

impl rte_event_1_1
{
	
	#[inline(always)]
	pub fn flow_id(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 20u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_flow_id(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(0usize, 20u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn sub_event_type(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(20usize, 8u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_sub_event_type(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(20usize, 8u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn event_type(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(28usize, 4u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_event_type(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(28usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn op(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(32usize, 2u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_op(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(32usize, 2u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn rsvd(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(34usize, 4u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_rsvd(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(34usize, 4u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn sched_type(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(38usize, 2u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_sched_type(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(38usize, 2u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(flow_id: u32, sub_event_type: u32, event_type: u32, op: u8, rsvd: u8, sched_type: u8) -> BindgenBitfieldUnit<[u8; 8usize], u32>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 8usize], u32> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 20u8, {
			let flow_id: u32 = unsafe { transmute(flow_id) };
			flow_id as u64
		});
		__bindgen_bitfield_unit.set(20usize, 8u8, {
			let sub_event_type: u32 = unsafe { transmute(sub_event_type) };
			sub_event_type as u64
		});
		__bindgen_bitfield_unit.set(28usize, 4u8, {
			let event_type: u32 = unsafe { transmute(event_type) };
			event_type as u64
		});
		__bindgen_bitfield_unit.set(32usize, 2u8, {
			let op: u8 = unsafe { transmute(op) };
			op as u64
		});
		__bindgen_bitfield_unit.set(34usize, 4u8, {
			let rsvd: u8 = unsafe { transmute(rsvd) };
			rsvd as u64
		});
		__bindgen_bitfield_unit.set(38usize, 2u8, {
			let sched_type: u8 = unsafe { transmute(sched_type) };
			sched_type as u64
		});
		__bindgen_bitfield_unit
	}
}
