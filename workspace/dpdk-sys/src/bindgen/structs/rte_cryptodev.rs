// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_cryptodev
{
	pub dequeue_burst: dequeue_pkt_burst_t,
	pub enqueue_burst: enqueue_pkt_burst_t,
	pub data: *mut rte_cryptodev_data,
	pub dev_ops: *mut rte_cryptodev_ops,
	pub feature_flags: u64,
	pub device: *mut rte_device,
	pub driver_id: u8,
	pub link_intr_cbs: rte_cryptodev_cb_list,
	pub security_ctx: *mut c_void,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 47usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_cryptodev
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_cryptodev
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_cryptodev {{ data: {:?}, dev_ops: {:?}, device: {:?}, link_intr_cbs: {:?}, security_ctx: {:?}, attached : {:?} }}", self.data, self.dev_ops, self.device, self.link_intr_cbs, self.security_ctx, self.attached())
	}
}

impl rte_cryptodev
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
	pub fn newbitfield_1(attached: u8) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let attached: u8 = unsafe { transmute(attached) };
			attached as u64
		});
		__bindgen_bitfield_unit
	}
}
