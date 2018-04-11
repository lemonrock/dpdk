// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct tle_udp_stream_param
{
	pub local_addr: sockaddr_storage,
	pub remote_addr: sockaddr_storage,
	pub recv_ev: *mut tle_event,
	pub recv_cb: tle_stream_cb,
	pub send_ev: *mut tle_event,
	pub send_cb: tle_stream_cb,
}

impl Default for tle_udp_stream_param
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for tle_udp_stream_param
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "tle_udp_stream_param {{ recv_ev: {:?}, recv_cb: {:?}, send_ev: {:?}, send_cb: {:?} }}", self.recv_ev, self.recv_cb, self.send_ev, self.send_cb)
	}
}
