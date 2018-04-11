// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_exit(exit_code: c_int, format: *const c_char, ...);
	pub fn rte_get_rx_ol_flag_list(mask: u64, buf: *mut c_char, buflen: usize) -> c_int;
	pub fn rte_get_tx_ol_flag_list(mask: u64, buf: *mut c_char, buflen: usize) -> c_int;
	pub fn rte_set_application_usage_hook(usage_func: rte_usage_hook_t) -> rte_usage_hook_t;
	pub fn rte_socket_id() -> c_uint;
	pub fn rte_stats_bitrate_calc(bitrate_data: *mut rte_stats_bitrates, port_id: u16) -> c_int;
	pub fn rte_stats_bitrate_create() -> *mut rte_stats_bitrates;
	pub fn rte_stats_bitrate_reg(bitrate_data: *mut rte_stats_bitrates) -> c_int;
	pub fn rust___mempool_generic_get(mp: *mut rte_mempool, obj_table: *mut *mut c_void, n: c_uint, cache: *mut rte_mempool_cache) -> c_int;
	pub fn rust___mempool_generic_put(mp: *mut rte_mempool, obj_table: *const *const c_void, n: c_uint, cache: *mut rte_mempool_cache);
	pub fn rust___mempool_get_header(obj: *mut c_void) -> *mut rte_mempool_objhdr;
	pub fn rust___mempool_get_trailer(obj: *mut c_void) -> *mut rte_mempool_objtlr;
	pub fn rust___rte_event_enqueue_burst(dev_id: u8, port_id: u8, ev: *const rte_event, nb_events: u16, fn_: event_enqueue_burst_t) -> u16;
	pub fn rust___rte_mbuf_refcnt_update(m: *mut rte_mbuf, value: i16) -> u16;
	pub fn rust___rte_raw_cksum(buf: *const c_void, len: usize, sum: u32) -> u32;
	pub fn rust___rte_raw_cksum_reduce(sum: u32) -> u16;
	pub fn rust___rte_ring_do_dequeue(r: *mut rte_ring, obj_table: *mut *mut c_void, n: c_uint, behavior: rte_ring_queue_behavior, is_sc: c_int, available: *mut c_uint) -> c_uint;
	pub fn rust___rte_ring_do_enqueue(r: *mut rte_ring, obj_table: *const *const c_void, n: c_uint, behavior: rte_ring_queue_behavior, is_sp: c_int, free_space: *mut c_uint) -> c_uint;
	pub fn rust___tle_dring_copy_objs(dst: *mut *const c_void, src: *const *const c_void, num: u32);
	pub fn rust___tle_dring_dequeue(dr: *mut tle_dring, head: u32, objs: *mut *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: u32) -> u32;
	pub fn rust___tle_dring_enqueue(dr: *mut tle_dring, head: u32, objs: *const *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: u32) -> u32;
}
