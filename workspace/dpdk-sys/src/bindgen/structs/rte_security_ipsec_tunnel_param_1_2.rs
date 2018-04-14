// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_security_ipsec_tunnel_param_1_2
{
	pub src_addr: in6_addr,
	pub dst_addr: in6_addr,
	pub dscp: u8,
	pub flabel: u32,
	pub hlimit: u8,
}

impl Default for rte_security_ipsec_tunnel_param_1_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_security_ipsec_tunnel_param_1_2
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_security_ipsec_tunnel_param_1_2 {{  }}")
	}
}
