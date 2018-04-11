// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_cryptodev_scheduler_ops
{
	pub slave_attach: rte_cryptodev_scheduler_slave_attach_t,
	pub slave_detach: rte_cryptodev_scheduler_slave_attach_t,
	pub scheduler_start: rte_cryptodev_scheduler_start_t,
	pub scheduler_stop: rte_cryptodev_scheduler_stop_t,
	pub config_queue_pair: rte_cryptodev_scheduler_config_queue_pair,
	pub create_private_ctx: rte_cryptodev_scheduler_create_private_ctx,
	pub option_set: rte_cryptodev_scheduler_config_option_set,
	pub option_get: rte_cryptodev_scheduler_config_option_get,
}

impl Default for rte_cryptodev_scheduler_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_cryptodev_scheduler_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_cryptodev_scheduler_ops {{  }}")
	}
}
