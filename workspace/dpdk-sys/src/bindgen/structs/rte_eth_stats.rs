// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_stats
{
	pub ipackets: u64,
	pub opackets: u64,
	pub ibytes: u64,
	pub obytes: u64,
	pub imissed: u64,
	pub ierrors: u64,
	pub oerrors: u64,
	pub rx_nombuf: u64,
	pub q_ipackets: [u64; 16usize],
	pub q_opackets: [u64; 16usize],
	pub q_ibytes: [u64; 16usize],
	pub q_obytes: [u64; 16usize],
	pub q_errors: [u64; 16usize],
}

impl Default for rte_eth_stats
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_stats
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_stats {{ q_ipackets: {:?}, q_opackets: {:?}, q_ibytes: {:?}, q_obytes: {:?}, q_errors: {:?} }}", self.q_ipackets, self.q_opackets, self.q_ibytes, self.q_obytes, self.q_errors)
	}
}
