// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct tle_dring
{
	pub flags: u32,
	pub __bindgen_padding_0: [u64; 7usize],
	pub prod: tle_dring__bindgen_ty_1,
	pub __bindgen_padding_1: [u64; 5usize],
	pub cons: tle_dring__bindgen_ty_2,
	pub __bindgen_padding_2: [u64; 5usize],
	pub dummy: tle_drb,
}

impl Default for tle_dring
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for tle_dring
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "tle_dring {{ prod: {:?}, cons: {:?}, dummy: {:?} }}", self.prod, self.cons, self.dummy)
	}
}