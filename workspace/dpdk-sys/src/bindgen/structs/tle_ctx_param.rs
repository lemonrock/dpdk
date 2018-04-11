// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct tle_ctx_param
{
	pub socket_id: i32,
	pub proto: u32,
	pub max_streams: u32,
	pub max_stream_rbufs: u32,
	pub max_stream_sbufs: u32,
	pub send_bulk_size: u32,
	pub flags: u32,
	pub lookup4: Option<unsafe extern "C" fn(opaque: *mut c_void, addr: *const in_addr, res: *mut tle_dest) -> c_int>,
	pub lookup4_data: *mut c_void,
	pub lookup6: Option<unsafe extern "C" fn(opaque: *mut c_void, addr: *const in6_addr, res: *mut tle_dest) -> c_int>,
	pub lookup6_data: *mut c_void,
	pub hash_alg: u32,
	pub __bindgen_padding_0: u64,
	pub secret_key: rte_xmm_t,
	pub icw: u32,
	pub timewait: u32,
	pub __bindgen_padding_1: u64,
}

impl Default for tle_ctx_param
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for tle_ctx_param
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "tle_ctx_param {{ lookup4: {:?}, lookup4_data: {:?}, lookup6: {:?}, lookup6_data: {:?}, secret_key: {:?} }}", self.lookup4, self.lookup4_data, self.lookup6, self.lookup6_data, self.secret_key)
	}
}
