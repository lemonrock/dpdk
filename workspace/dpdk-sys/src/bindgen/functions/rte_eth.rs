// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_eth_add_first_rx_callback(port_id: u16, queue_id: u16, fn_: rte_rx_callback_fn, user_param: *mut c_void) -> *const rte_eth_rxtx_callback;
	pub fn rte_eth_add_rx_callback(port_id: u16, queue_id: u16, fn_: rte_rx_callback_fn, user_param: *mut c_void) -> *const rte_eth_rxtx_callback;
	pub fn rte_eth_add_tx_callback(port_id: u16, queue_id: u16, fn_: rte_tx_callback_fn, user_param: *mut c_void) -> *const rte_eth_rxtx_callback;
	pub fn rte_eth_allmulticast_disable(port_id: u16);
	pub fn rte_eth_allmulticast_enable(port_id: u16);
	pub fn rte_eth_allmulticast_get(port_id: u16) -> c_int;
	pub fn rte_eth_devargs_parse(devargs: *const c_char, eth_devargs: *mut rte_eth_devargs) -> c_int;
	pub fn rte_eth_dma_zone_reserve(eth_dev: *const rte_eth_dev, name: *const c_char, queue_id: u16, size: usize, align: c_uint, socket_id: c_int) -> *const rte_memzone;
	pub fn rte_eth_find_next(port_id: u16) -> u16;
	pub fn rte_eth_find_next_owned_by(port_id: u16, owner_id: u64) -> u64;
	pub fn rte_eth_from_ring(r: *mut rte_ring) -> c_int;
	pub fn rte_eth_from_rings(name: *const c_char, rx_queues: *const *const rte_ring, nb_rx_queues: c_uint, tx_queues: *const *const rte_ring, nb_tx_queues: c_uint, numa_node: c_uint) -> c_int;
	pub fn rte_eth_led_off(port_id: u16) -> c_int;
	pub fn rte_eth_led_on(port_id: u16) -> c_int;
	pub fn rte_eth_link_get(port_id: u16, link: *mut rte_eth_link);
	pub fn rte_eth_link_get_nowait(port_id: u16, link: *mut rte_eth_link);
	pub fn rte_eth_macaddr_get(port_id: u16, mac_addr: *mut ether_addr);
	pub fn rte_eth_mirror_rule_reset(port_id: u16, rule_id: u8) -> c_int;
	pub fn rte_eth_mirror_rule_set(port_id: u16, mirror_conf: *mut rte_eth_mirror_conf, rule_id: u8, on: u8) -> c_int;
	pub fn rte_eth_promiscuous_disable(port_id: u16);
	pub fn rte_eth_promiscuous_enable(port_id: u16);
	pub fn rte_eth_promiscuous_get(port_id: u16) -> c_int;
	pub fn rte_eth_remove_rx_callback(port_id: u16, queue_id: u16, user_cb: *const rte_eth_rxtx_callback) -> c_int;
	pub fn rte_eth_remove_tx_callback(port_id: u16, queue_id: u16, user_cb: *const rte_eth_rxtx_callback) -> c_int;
	pub fn rte_eth_rx_queue_info_get(port_id: u16, queue_id: u16, qinfo: *mut rte_eth_rxq_info) -> c_int;
	pub fn rte_eth_rx_queue_setup(port_id: u16, rx_queue_id: u16, nb_rx_desc: u16, socket_id: c_uint, rx_conf: *const rte_eth_rxconf, mb_pool: *mut rte_mempool) -> c_int;
	pub fn rte_eth_set_queue_rate_limit(port_id: u16, queue_idx: u16, tx_rate: u16) -> c_int;
	pub fn rte_eth_speed_bitflag(speed: u32, duplex: c_int) -> u32;
	pub fn rte_eth_stats_get(port_id: u16, stats: *mut rte_eth_stats) -> c_int;
	pub fn rte_eth_stats_reset(port_id: u16) -> c_int;
	pub fn rte_eth_switch_domain_alloc(domain_id: *mut u16) -> c_int;
	pub fn rte_eth_switch_domain_free(domain_id: u16) -> c_int;
	pub fn rte_eth_timesync_adjust_time(port_id: u16, delta: i64) -> c_int;
	pub fn rte_eth_timesync_disable(port_id: u16) -> c_int;
	pub fn rte_eth_timesync_enable(port_id: u16) -> c_int;
	pub fn rte_eth_timesync_read_rx_timestamp(port_id: u16, timestamp: *mut timespec, flags: u32) -> c_int;
	pub fn rte_eth_timesync_read_time(port_id: u16, time: *mut timespec) -> c_int;
	pub fn rte_eth_timesync_read_tx_timestamp(port_id: u16, timestamp: *mut timespec) -> c_int;
	pub fn rte_eth_timesync_write_time(port_id: u16, time: *const timespec) -> c_int;
	pub fn rte_eth_tx_buffer_count_callback(pkts: *mut *mut rte_mbuf, unsent: u16, userdata: *mut c_void);
	pub fn rte_eth_tx_buffer_drop_callback(pkts: *mut *mut rte_mbuf, unsent: u16, userdata: *mut c_void);
	pub fn rte_eth_tx_buffer_init(buffer: *mut rte_eth_dev_tx_buffer, size: u16) -> c_int;
	pub fn rte_eth_tx_buffer_set_err_callback(buffer: *mut rte_eth_dev_tx_buffer, callback: buffer_tx_error_fn, userdata: *mut c_void) -> c_int;
	pub fn rte_eth_tx_done_cleanup(port_id: u16, queue_id: u16, free_cnt: u32) -> c_int;
	pub fn rte_eth_tx_queue_info_get(port_id: u16, queue_id: u16, qinfo: *mut rte_eth_txq_info) -> c_int;
	pub fn rte_eth_tx_queue_setup(port_id: u16, tx_queue_id: u16, nb_tx_desc: u16, socket_id: c_uint, tx_conf: *const rte_eth_txconf) -> c_int;
	pub fn rte_eth_xstats_get(port_id: u16, xstats: *mut rte_eth_xstat, n: c_uint) -> c_int;
	pub fn rte_eth_xstats_get_by_id(port_id: u16, ids: *const u64, values: *mut u64, size: c_uint) -> c_int;
	pub fn rte_eth_xstats_get_id_by_name(port_id: u16, xstat_name: *const c_char, id: *mut u64) -> c_int;
	pub fn rte_eth_xstats_get_names(port_id: u16, xstats_names: *mut rte_eth_xstat_name, size: c_uint) -> c_int;
	pub fn rte_eth_xstats_get_names_by_id(port_id: u16, xstats_names: *mut rte_eth_xstat_name, size: c_uint, ids: *mut u64) -> c_int;
	pub fn rte_eth_xstats_reset(port_id: u16);
}
