// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_bbdev
{
	pub enqueue_enc_ops: rte_bbdev_enqueue_enc_ops_t,
	pub enqueue_dec_ops: rte_bbdev_enqueue_dec_ops_t,
	pub dequeue_enc_ops: rte_bbdev_dequeue_enc_ops_t,
	pub dequeue_dec_ops: rte_bbdev_dequeue_dec_ops_t,
	pub dev_ops: *const rte_bbdev_ops,
	pub data: *mut rte_bbdev_data,
	pub state: rte_bbdev_state,
	pub device: *mut rte_device,
	pub list_cbs: rte_bbdev_cb_list,
	pub intr_handle: *mut rte_intr_handle,
	pub __bindgen_padding_0: [u64; 5usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_bbdev
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bbdev {{ dev_ops: {:?}, data: {:?}, state: {:?}, device: {:?}, list_cbs: {:?}, intr_handle: {:?} }}", self.dev_ops, self.data, self.state, self.device, self.list_cbs, self.intr_handle)
	}
}
