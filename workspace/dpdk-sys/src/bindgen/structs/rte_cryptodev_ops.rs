// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_cryptodev_ops
{
	pub dev_configure: cryptodev_configure_t,
	pub dev_start: cryptodev_start_t,
	pub dev_stop: cryptodev_stop_t,
	pub dev_close: cryptodev_close_t,
	pub dev_infos_get: cryptodev_info_get_t,
	pub stats_get: cryptodev_stats_get_t,
	pub stats_reset: cryptodev_stats_reset_t,
	pub queue_pair_setup: cryptodev_queue_pair_setup_t,
	pub queue_pair_release: cryptodev_queue_pair_release_t,
	pub queue_pair_start: cryptodev_queue_pair_start_t,
	pub queue_pair_stop: cryptodev_queue_pair_stop_t,
	pub queue_pair_count: cryptodev_queue_pair_count_t,
	pub session_get_size: cryptodev_sym_get_session_private_size_t,
	pub session_configure: cryptodev_sym_configure_session_t,
	pub session_clear: cryptodev_sym_free_session_t,
	pub qp_attach_session: cryptodev_sym_queue_pair_attach_session_t,
	pub qp_detach_session: cryptodev_sym_queue_pair_detach_session_t,
}

impl Default for rte_cryptodev_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_cryptodev_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_cryptodev_ops {{  }}")
	}
}
