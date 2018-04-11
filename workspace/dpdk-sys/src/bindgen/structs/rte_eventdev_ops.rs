// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_eventdev_ops
{
	pub dev_infos_get: eventdev_info_get_t,
	pub dev_configure: eventdev_configure_t,
	pub dev_start: eventdev_start_t,
	pub dev_stop: eventdev_stop_t,
	pub dev_close: eventdev_close_t,
	pub queue_def_conf: eventdev_queue_default_conf_get_t,
	pub queue_setup: eventdev_queue_setup_t,
	pub queue_release: eventdev_queue_release_t,
	pub port_def_conf: eventdev_port_default_conf_get_t,
	pub port_setup: eventdev_port_setup_t,
	pub port_release: eventdev_port_release_t,
	pub port_link: eventdev_port_link_t,
	pub port_unlink: eventdev_port_unlink_t,
	pub timeout_ticks: eventdev_dequeue_timeout_ticks_t,
	pub dump: eventdev_dump_t,
	pub xstats_get: eventdev_xstats_get_t,
	pub xstats_get_names: eventdev_xstats_get_names_t,
	pub xstats_get_by_name: eventdev_xstats_get_by_name,
	pub xstats_reset: eventdev_xstats_reset_t,
	pub eth_rx_adapter_caps_get: eventdev_eth_rx_adapter_caps_get_t,
	pub eth_rx_adapter_queue_add: eventdev_eth_rx_adapter_queue_add_t,
	pub eth_rx_adapter_queue_del: eventdev_eth_rx_adapter_queue_del_t,
	pub eth_rx_adapter_start: eventdev_eth_rx_adapter_start_t,
	pub eth_rx_adapter_stop: eventdev_eth_rx_adapter_stop_t,
	pub eth_rx_adapter_stats_get: eventdev_eth_rx_adapter_stats_get,
	pub eth_rx_adapter_stats_reset: eventdev_eth_rx_adapter_stats_reset,
	pub dev_selftest: eventdev_selftest,
}

impl Default for rte_eventdev_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eventdev_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eventdev_ops {{  }}")
	}
}
