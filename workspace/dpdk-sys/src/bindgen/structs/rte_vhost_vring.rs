// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_vhost_vring
{
	pub desc: *mut vring_desc,
	pub avail: *mut vring_avail,
	pub used: *mut vring_used,
	pub log_guest_addr: u64,
	pub callfd: c_int,
	pub kickfd: c_int,
	pub size: u16,
}

impl Default for rte_vhost_vring
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_vhost_vring
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_vhost_vring {{ desc: {:?}, avail: {:?}, used: {:?} }}", self.desc, self.avail, self.used)
	}
}
