// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct tle_dev_param
{
	pub rx_offload: u64,
	pub tx_offload: u64,
	pub local_addr4: in_addr,
	pub local_addr6: in6_addr,
	pub bl4: tle_bl_port,
	pub bl6: tle_bl_port,
}

impl Default for tle_dev_param
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for tle_dev_param
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "tle_dev_param {{ bl4: {:?}, bl6: {:?} }}", self.bl4, self.bl6)
	}
}
