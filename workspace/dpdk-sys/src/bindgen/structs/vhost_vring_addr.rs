// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct vhost_vring_addr
{
	pub index: c_uint,
	pub flags: c_uint,
	pub desc_user_addr: __u64,
	pub used_user_addr: __u64,
	pub avail_user_addr: __u64,
	pub log_guest_addr: __u64,
}

impl Default for vhost_vring_addr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for vhost_vring_addr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "vhost_vring_addr {{  }}")
	}
}
