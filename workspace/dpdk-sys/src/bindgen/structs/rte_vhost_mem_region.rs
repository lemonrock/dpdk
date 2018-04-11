// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_vhost_mem_region
{
	pub guest_phys_addr: u64,
	pub guest_user_addr: u64,
	pub host_user_addr: u64,
	pub size: u64,
	pub mmap_addr: *mut c_void,
	pub mmap_size: u64,
	pub fd: c_int,
}

impl Default for rte_vhost_mem_region
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_vhost_mem_region
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_vhost_mem_region {{ mmap_addr: {:?} }}", self.mmap_addr)
	}
}
