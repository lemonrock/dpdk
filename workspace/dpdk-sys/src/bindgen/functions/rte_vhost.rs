// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_vhost_avail_entries(vid: c_int, queue_id: u16) -> u16;
	pub fn rte_vhost_dequeue_burst(vid: c_int, queue_id: u16, mbuf_pool: *mut rte_mempool, pkts: *mut *mut rte_mbuf, count: u16) -> u16;
	pub fn rte_vhost_driver_attach_vdpa_device(path: *const c_char, did: c_int) -> c_int;
	pub fn rte_vhost_driver_callback_register(path: *const c_char, ops: *const vhost_device_ops) -> c_int;
	pub fn rte_vhost_driver_detach_vdpa_device(path: *const c_char) -> c_int;
	pub fn rte_vhost_driver_disable_features(path: *const c_char, features: u64) -> c_int;
	pub fn rte_vhost_driver_enable_features(path: *const c_char, features: u64) -> c_int;
	pub fn rte_vhost_driver_get_features(path: *const c_char, features: *mut u64) -> c_int;
	pub fn rte_vhost_driver_get_protocol_features(path: *const c_char, protocol_features: *mut u64) -> c_int;
	pub fn rte_vhost_driver_get_queue_num(path: *const c_char, queue_num: *mut u32) -> c_int;
	pub fn rte_vhost_driver_get_vdpa_device_id(path: *const c_char) -> c_int;
	pub fn rte_vhost_driver_register(path: *const c_char, flags: u64) -> c_int;
	pub fn rte_vhost_driver_set_features(path: *const c_char, features: u64) -> c_int;
	pub fn rte_vhost_driver_start(path: *const c_char) -> c_int;
	pub fn rte_vhost_driver_unregister(path: *const c_char) -> c_int;
	pub fn rte_vhost_enable_guest_notification(vid: c_int, queue_id: u16, enable: c_int) -> c_int;
	pub fn rte_vhost_enqueue_burst(vid: c_int, queue_id: u16, pkts: *mut *mut rte_mbuf, count: u16) -> u16;
	pub fn rte_vhost_get_ifname(vid: c_int, buf: *mut c_char, len: usize) -> c_int;
	pub fn rte_vhost_get_log_base(vid: c_int, log_base: *mut u64, log_size: *mut u64) -> c_int;
	pub fn rte_vhost_get_mem_table(vid: c_int, mem: *mut *mut rte_vhost_memory) -> c_int;
	pub fn rte_vhost_get_mtu(vid: c_int, mtu: *mut u16) -> c_int;
	pub fn rte_vhost_get_negotiated_features(vid: c_int, features: *mut u64) -> c_int;
	pub fn rte_vhost_get_numa_node(vid: c_int) -> c_int;
	pub fn rte_vhost_get_queue_num(vid: c_int) -> u32;
	pub fn rte_vhost_get_vdpa_device_id(vid: c_int) -> c_int;
	pub fn rte_vhost_get_vhost_vring(vid: c_int, vring_idx: u16, vring: *mut rte_vhost_vring) -> c_int;
	pub fn rte_vhost_get_vring_base(vid: c_int, queue_id: u16, last_avail_idx: *mut u16, last_used_idx: *mut u16) -> c_int;
	pub fn rte_vhost_get_vring_num(vid: c_int) -> u16;
	pub fn rte_vhost_log_used_vring(vid: c_int, vring_idx: u16, offset: u64, len: u64);
	pub fn rte_vhost_log_write(vid: c_int, addr: u64, len: u64);
	pub fn rte_vhost_rx_queue_count(vid: c_int, qid: u16) -> u32;
	pub fn rte_vhost_set_vring_base(vid: c_int, queue_id: u16, last_avail_idx: u16, last_used_idx: u16) -> c_int;
	pub fn rte_vhost_vring_call(vid: c_int, vring_idx: u16) -> c_int;
}
