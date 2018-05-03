// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_event_timer
{
	pub ev: rte_event,
	pub state: rte_event_timer_state,
	pub timeout_ticks: u64,
	pub impl_opaque: [u64; 2usize],
	pub user_meta: IncompleteArrayField<u8>,
	pub __bindgen_padding_0: [u64; 2usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_event_timer
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_timer
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_timer {{ ev: {:?}, state: {:?}, impl_opaque: {:?}, user_meta: {:?} }}", self.ev, self.state, self.impl_opaque, self.user_meta)
	}
}
