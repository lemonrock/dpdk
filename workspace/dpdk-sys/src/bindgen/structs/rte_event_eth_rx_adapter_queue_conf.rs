// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_event_eth_rx_adapter_queue_conf
{
	pub rx_queue_flags: u32,
	pub servicing_weight: u16,
	pub ev: rte_event,
}

impl Default for rte_event_eth_rx_adapter_queue_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_eth_rx_adapter_queue_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_eth_rx_adapter_queue_conf {{ ev: {:?} }}", self.ev)
	}
}
