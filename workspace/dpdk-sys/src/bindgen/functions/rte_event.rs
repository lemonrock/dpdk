// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_event_dequeue_timeout_ticks(dev_id: u8, ns: u64, timeout_ticks: *mut u64) -> c_int;
	pub fn rte_event_dev_attr_get(dev_id: u8, attr_id: u32, attr_value: *mut u32) -> c_int;
	pub fn rte_event_dev_close(dev_id: u8) -> c_int;
	pub fn rte_event_dev_configure(dev_id: u8, dev_conf: *const rte_event_dev_config) -> c_int;
	pub fn rte_event_dev_count() -> u8;
	pub fn rte_event_dev_dump(dev_id: u8, f: *mut FILE) -> c_int;
	pub fn rte_event_dev_get_dev_id(name: *const c_char) -> c_int;
	pub fn rte_event_dev_info_get(dev_id: u8, dev_info: *mut rte_event_dev_info) -> c_int;
	pub fn rte_event_dev_selftest(dev_id: u8) -> c_int;
	pub fn rte_event_dev_service_id_get(dev_id: u8, service_id: *mut u32) -> c_int;
	pub fn rte_event_dev_socket_id(dev_id: u8) -> c_int;
	pub fn rte_event_dev_start(dev_id: u8) -> c_int;
	pub fn rte_event_dev_stop(dev_id: u8);
	pub fn rte_event_dev_stop_flush_callback_register(dev_id: u8, callback: eventdev_stop_flush_t, userdata: *mut c_void) -> c_int;
	pub fn rte_event_dev_xstats_by_name_get(dev_id: u8, name: *const c_char, id: *mut c_uint) -> u64;
	pub fn rte_event_dev_xstats_get(dev_id: u8, mode: rte_event_dev_xstats_mode, queue_port_id: u8, ids: *const c_uint, values: *mut u64, n: c_uint) -> c_int;
	pub fn rte_event_dev_xstats_names_get(dev_id: u8, mode: rte_event_dev_xstats_mode, queue_port_id: u8, xstats_names: *mut rte_event_dev_xstats_name, ids: *mut c_uint, size: c_uint) -> c_int;
	pub fn rte_event_dev_xstats_reset(dev_id: u8, mode: rte_event_dev_xstats_mode, queue_port_id: i16, ids: *const u32, nb_ids: u32) -> c_int;
	pub fn rte_event_eth_rx_adapter_caps_get(dev_id: u8, eth_port_id: u8, caps: *mut u32) -> c_int;
	pub fn rte_event_eth_rx_adapter_create(id: u8, dev_id: u8, port_config: *mut rte_event_port_conf) -> c_int;
	pub fn rte_event_eth_rx_adapter_create_ext(id: u8, dev_id: u8, conf_cb: rte_event_eth_rx_adapter_conf_cb, conf_arg: *mut c_void) -> c_int;
	pub fn rte_event_eth_rx_adapter_free(id: u8) -> c_int;
	pub fn rte_event_eth_rx_adapter_queue_add(id: u8, eth_dev_id: u8, rx_queue_id: i32, conf: *const rte_event_eth_rx_adapter_queue_conf) -> c_int;
	pub fn rte_event_eth_rx_adapter_queue_del(id: u8, eth_dev_id: u8, rx_queue_id: i32) -> c_int;
	pub fn rte_event_eth_rx_adapter_service_id_get(id: u8, service_id: *mut u32) -> c_int;
	pub fn rte_event_eth_rx_adapter_start(id: u8) -> c_int;
	pub fn rte_event_eth_rx_adapter_stats_get(id: u8, stats: *mut rte_event_eth_rx_adapter_stats) -> c_int;
	pub fn rte_event_eth_rx_adapter_stats_reset(id: u8) -> c_int;
	pub fn rte_event_eth_rx_adapter_stop(id: u8) -> c_int;
	pub fn rte_event_pmd_allocate(name: *const c_char, socket_id: c_int) -> *mut rte_eventdev;
	pub fn rte_event_pmd_release(eventdev: *mut rte_eventdev) -> c_int;
	pub fn rte_event_port_attr_get(dev_id: u8, port_id: u8, attr_id: u32, attr_value: *mut u32) -> c_int;
	pub fn rte_event_port_default_conf_get(dev_id: u8, port_id: u8, port_conf: *mut rte_event_port_conf) -> c_int;
	pub fn rte_event_port_link(dev_id: u8, port_id: u8, queues: *const u8, priorities: *const u8, nb_links: u16) -> c_int;
	pub fn rte_event_port_links_get(dev_id: u8, port_id: u8, queues: *mut u8, priorities: *mut u8) -> c_int;
	pub fn rte_event_port_setup(dev_id: u8, port_id: u8, port_conf: *const rte_event_port_conf) -> c_int;
	pub fn rte_event_port_unlink(dev_id: u8, port_id: u8, queues: *mut u8, nb_unlinks: u16) -> c_int;
	pub fn rte_event_queue_attr_get(dev_id: u8, queue_id: u8, attr_id: u32, attr_value: *mut u32) -> c_int;
	pub fn rte_event_queue_default_conf_get(dev_id: u8, queue_id: u8, queue_conf: *mut rte_event_queue_conf) -> c_int;
	pub fn rte_event_queue_setup(dev_id: u8, queue_id: u8, queue_conf: *const rte_event_queue_conf) -> c_int;
	pub fn rte_event_ring_create(name: *const c_char, count: c_uint, socket_id: c_int, flags: c_uint) -> *mut rte_event_ring;
	pub fn rte_event_ring_free(r: *mut rte_event_ring);
	pub fn rte_event_ring_init(r: *mut rte_event_ring, name: *const c_char, count: c_uint, flags: c_uint) -> c_int;
	pub fn rte_event_ring_lookup(name: *const c_char) -> *mut rte_event_ring;
	pub fn rte_event_timer_adapter_caps_get(dev_id: u8, caps: *mut u32) -> c_int;
	pub fn rte_event_timer_adapter_create(conf: *const rte_event_timer_adapter_conf) -> *mut rte_event_timer_adapter;
	pub fn rte_event_timer_adapter_create_ext(conf: *const rte_event_timer_adapter_conf, conf_cb: rte_event_timer_adapter_port_conf_cb_t, conf_arg: *mut c_void) -> *mut rte_event_timer_adapter;
	pub fn rte_event_timer_adapter_free(adapter: *mut rte_event_timer_adapter) -> c_int;
	pub fn rte_event_timer_adapter_get_info(adapter: *const rte_event_timer_adapter, adapter_info: *mut rte_event_timer_adapter_info) -> c_int;
	pub fn rte_event_timer_adapter_lookup(adapter_id: u16) -> *mut rte_event_timer_adapter;
	pub fn rte_event_timer_adapter_service_id_get(adapter: *mut rte_event_timer_adapter, service_id: *mut u32) -> c_int;
	pub fn rte_event_timer_adapter_start(adapter: *const rte_event_timer_adapter) -> c_int;
	pub fn rte_event_timer_adapter_stats_get(adapter: *mut rte_event_timer_adapter, stats: *mut rte_event_timer_adapter_stats) -> c_int;
	pub fn rte_event_timer_adapter_stats_reset(adapter: *mut rte_event_timer_adapter) -> c_int;
	pub fn rte_event_timer_adapter_stop(adapter: *const rte_event_timer_adapter) -> c_int;
	pub fn rust___rte_event_enqueue_burst(dev_id: u8, port_id: u8, ev: *const rte_event, nb_events: u16, fn_: event_enqueue_burst_t) -> u16;
}
