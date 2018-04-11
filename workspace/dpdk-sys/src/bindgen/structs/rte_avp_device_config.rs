// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct rte_avp_device_config
{
	pub device_id: u64,
	pub driver_type: u32,
	pub driver_version: u32,
	pub features: u32,
	pub num_tx_queues: u16,
	pub num_rx_queues: u16,
	pub if_up: u8,
}

impl Default for rte_avp_device_config
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_avp_device_config
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_avp_device_config {{  }}")
	}
}
