// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_bbdev_allocate(name: *const c_char) -> *mut rte_bbdev;
	pub fn rte_bbdev_callback_register(dev_id: u16, event: rte_bbdev_event_type, cb_fn: rte_bbdev_cb_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_bbdev_callback_unregister(dev_id: u16, event: rte_bbdev_event_type, cb_fn: rte_bbdev_cb_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_bbdev_close(dev_id: u16) -> c_int;
	pub fn rte_bbdev_count() -> u16;
	pub fn rte_bbdev_find_next(dev_id: u16) -> u16;
	pub fn rte_bbdev_get_named_dev(name: *const c_char) -> *mut rte_bbdev;
	pub fn rte_bbdev_info_get(dev_id: u16, dev_info: *mut rte_bbdev_info) -> c_int;
	pub fn rte_bbdev_intr_enable(dev_id: u16) -> c_int;
	pub fn rte_bbdev_is_valid(dev_id: u16) -> bool;
	pub fn rte_bbdev_op_pool_create(name: *const c_char, type_: rte_bbdev_op_type, num_elements: c_uint, cache_size: c_uint, socket_id: c_int) -> *mut rte_mempool;
	pub fn rte_bbdev_op_type_str(op_type: rte_bbdev_op_type) -> *const c_char;
	pub fn rte_bbdev_pmd_callback_process(dev: *mut rte_bbdev, event: rte_bbdev_event_type, ret_param: *mut c_void);
	pub fn rte_bbdev_queue_configure(dev_id: u16, queue_id: u16, conf: *const rte_bbdev_queue_conf) -> c_int;
	pub fn rte_bbdev_queue_info_get(dev_id: u16, queue_id: u16, queue_info: *mut rte_bbdev_queue_info) -> c_int;
	pub fn rte_bbdev_queue_intr_ctl(dev_id: u16, queue_id: u16, epfd: c_int, op: c_int, data: *mut c_void) -> c_int;
	pub fn rte_bbdev_queue_intr_disable(dev_id: u16, queue_id: u16) -> c_int;
	pub fn rte_bbdev_queue_intr_enable(dev_id: u16, queue_id: u16) -> c_int;
	pub fn rte_bbdev_queue_start(dev_id: u16, queue_id: u16) -> c_int;
	pub fn rte_bbdev_queue_stop(dev_id: u16, queue_id: u16) -> c_int;
	pub fn rte_bbdev_release(bbdev: *mut rte_bbdev) -> c_int;
	pub fn rte_bbdev_setup_queues(dev_id: u16, num_queues: u16, socket_id: c_int) -> c_int;
	pub fn rte_bbdev_start(dev_id: u16) -> c_int;
	pub fn rte_bbdev_stats_get(dev_id: u16, stats: *mut rte_bbdev_stats) -> c_int;
	pub fn rte_bbdev_stats_reset(dev_id: u16) -> c_int;
	pub fn rte_bbdev_stop(dev_id: u16) -> c_int;
}
