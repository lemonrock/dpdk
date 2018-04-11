// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_eventdev
{
	pub enqueue: event_enqueue_t,
	pub enqueue_burst: event_enqueue_burst_t,
	pub enqueue_new_burst: event_enqueue_burst_t,
	pub enqueue_forward_burst: event_enqueue_burst_t,
	pub dequeue: event_dequeue_t,
	pub dequeue_burst: event_dequeue_burst_t,
	pub data: *mut rte_eventdev_data,
	pub dev_ops: *const rte_eventdev_ops,
	pub dev: *mut rte_device,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 55usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_eventdev
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eventdev
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eventdev {{ data: {:?}, dev_ops: {:?}, dev: {:?}, attached : {:?} }}", self.data, self.dev_ops, self.dev, self.attached())
	}
}

impl rte_eventdev
{
	
	#[inline(always)]
	pub fn attached(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_attached(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(attached: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let attached: u8 = unsafe { transmute(attached) };
			attached as u64
		});
		__bindgen_bitfield_unit
	}
}
