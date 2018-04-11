// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_ntuple_filter
{
	pub flags: u16,
	pub dst_ip: u32,
	pub dst_ip_mask: u32,
	pub src_ip: u32,
	pub src_ip_mask: u32,
	pub dst_port: u16,
	pub dst_port_mask: u16,
	pub src_port: u16,
	pub src_port_mask: u16,
	pub proto: u8,
	pub proto_mask: u8,
	pub tcp_flags: u8,
	pub priority: u16,
	pub queue: u16,
}

impl Default for rte_eth_ntuple_filter
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_ntuple_filter
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_ntuple_filter {{  }}")
	}
}
