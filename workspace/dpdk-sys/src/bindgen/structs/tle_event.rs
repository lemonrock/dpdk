// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct tle_event
{
	pub ql: tle_event_1,
	pub head: *mut tle_evq,
	pub data: *const c_void,
	pub state: tle_ev_state,
	pub __bindgen_padding_0: [u32; 7usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for tle_event
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for tle_event
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "tle_event {{ ql: {:?}, head: {:?}, data: {:?}, state: {:?} }}", self.ql, self.head, self.data, self.state)
	}
}

impl PartialEq for tle_event
{
	#[inline(always)]
	fn eq(&self, other: &tle_event) -> bool
	{
		self.ql == other.ql && self.head == other.head && self.data == other.data && self.state == other.state
	}
}
