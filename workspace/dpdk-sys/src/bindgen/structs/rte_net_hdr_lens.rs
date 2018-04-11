// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_net_hdr_lens
{
	pub l2_len: u8,
	pub l3_len: u8,
	pub l4_len: u8,
	pub tunnel_len: u8,
	pub inner_l2_len: u8,
	pub inner_l3_len: u8,
	pub inner_l4_len: u8,
}

impl Default for rte_net_hdr_lens
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_net_hdr_lens
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_net_hdr_lens {{  }}")
	}
}
