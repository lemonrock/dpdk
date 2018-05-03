// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_ctrl_thread_create(thread: *mut pthread_t, name: *const c_char, attr: *const pthread_attr_t, start_routine: Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>, arg: *mut c_void) -> c_int;
	pub fn rte_dev_event_callback_register(device_name: *const c_char, cb_fn: rte_dev_event_cb_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_dev_event_callback_unregister(device_name: *const c_char, cb_fn: rte_dev_event_cb_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_dev_event_monitor_start() -> c_int;
	pub fn rte_dev_event_monitor_stop() -> c_int;
	pub fn rte_devargs_add(devtype: rte_devtype, devargs_str: *const c_char) -> c_int;
	pub fn rte_devargs_dump(f: *mut FILE);
	pub fn rte_devargs_insert(da: *mut rte_devargs) -> c_int;
	pub fn rte_devargs_next(busname: *const c_char, start: *const rte_devargs) -> *mut rte_devargs;
	pub fn rte_devargs_parse(da: *mut rte_devargs, format: *const c_char, ...) -> c_int;
	pub fn rte_devargs_remove(busname: *const c_char, devname: *const c_char) -> c_int;
	pub fn rte_devargs_type_count(devtype: rte_devtype) -> c_uint;
	pub fn rte_exit(exit_code: c_int, format: *const c_char, ...);
	pub fn rte_fbarray_attach(arr: *mut rte_fbarray) -> c_int;
	pub fn rte_fbarray_destroy(arr: *mut rte_fbarray) -> c_int;
	pub fn rte_fbarray_detach(arr: *mut rte_fbarray) -> c_int;
	pub fn rte_fbarray_dump_metadata(arr: *mut rte_fbarray, f: *mut FILE);
	pub fn rte_fbarray_find_contig_free(arr: *mut rte_fbarray, start: c_uint) -> c_int;
	pub fn rte_fbarray_find_contig_used(arr: *mut rte_fbarray, start: c_uint) -> c_int;
	pub fn rte_fbarray_find_idx(arr: *const rte_fbarray, elt: *const c_void) -> c_int;
	pub fn rte_fbarray_find_next_free(arr: *mut rte_fbarray, start: c_uint) -> c_int;
	pub fn rte_fbarray_find_next_n_free(arr: *mut rte_fbarray, start: c_uint, n: c_uint) -> c_int;
	pub fn rte_fbarray_find_next_n_used(arr: *mut rte_fbarray, start: c_uint, n: c_uint) -> c_int;
	pub fn rte_fbarray_find_next_used(arr: *mut rte_fbarray, start: c_uint) -> c_int;
	pub fn rte_fbarray_get(arr: *const rte_fbarray, idx: c_uint) -> *mut c_void;
	pub fn rte_fbarray_init(arr: *mut rte_fbarray, name: *const c_char, len: c_uint, elt_sz: c_uint) -> c_int;
	pub fn rte_fbarray_is_used(arr: *mut rte_fbarray, idx: c_uint) -> c_int;
	pub fn rte_fbarray_set_free(arr: *mut rte_fbarray, idx: c_uint) -> c_int;
	pub fn rte_fbarray_set_used(arr: *mut rte_fbarray, idx: c_uint) -> c_int;
	pub fn rte_memseg_contig_walk(func: rte_memseg_contig_walk_t, arg: *mut c_void) -> c_int;
	pub fn rte_memseg_list_walk(func: rte_memseg_list_walk_t, arg: *mut c_void) -> c_int;
	pub fn rte_memseg_walk(func: rte_memseg_walk_t, arg: *mut c_void) -> c_int;
	pub fn rte_set_application_usage_hook(usage_func: rte_usage_hook_t) -> rte_usage_hook_t;
	pub fn rte_socket_count() -> c_uint;
	pub fn rte_socket_id() -> c_uint;
	pub fn rte_socket_id_by_idx(idx: c_uint) -> c_int;
	pub fn rust_RTE_DEV_TO_VDEV(device: *mut rte_device) -> *mut rte_vdev_device;
	pub fn rust___rte_pktmbuf_free_direct(m: *mut rte_mbuf);
	pub fn rust___rte_pktmbuf_free_extbuf(m: *mut rte_mbuf);
}
