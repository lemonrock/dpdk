// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_dev_tx_buffer
{
	pub error_callback: buffer_tx_error_fn,
	pub error_userdata: *mut c_void,
	pub size: u16,
	pub length: u16,
	pub pkts: IncompleteArrayField<*mut rte_mbuf>,
}

impl Default for rte_eth_dev_tx_buffer
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_dev_tx_buffer
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_dev_tx_buffer {{ error_userdata: {:?}, pkts: {:?} }}", self.error_userdata, self.pkts)
	}
}
