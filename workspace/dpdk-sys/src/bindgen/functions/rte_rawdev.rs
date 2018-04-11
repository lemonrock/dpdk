// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_rawdev_close(dev_id: u16) -> c_int;
	pub fn rte_rawdev_configure(dev_id: u16, dev_conf: *mut rte_rawdev_info) -> c_int;
	pub fn rte_rawdev_count() -> u8;
	pub fn rte_rawdev_dequeue_buffers(dev_id: u16, buffers: *mut *mut rte_rawdev_buf, count: c_uint, context: rte_rawdev_obj_t) -> c_int;
	pub fn rte_rawdev_dump(dev_id: u16, f: *mut FILE) -> c_int;
	pub fn rte_rawdev_enqueue_buffers(dev_id: u16, buffers: *mut *mut rte_rawdev_buf, count: c_uint, context: rte_rawdev_obj_t) -> c_int;
	pub fn rte_rawdev_firmware_load(dev_id: u16, firmware_image: rte_rawdev_obj_t) -> c_int;
	pub fn rte_rawdev_firmware_status_get(dev_id: u16, status_info: rte_rawdev_obj_t) -> c_int;
	pub fn rte_rawdev_firmware_unload(dev_id: u16) -> c_int;
	pub fn rte_rawdev_firmware_version_get(dev_id: u16, version_info: rte_rawdev_obj_t) -> c_int;
	pub fn rte_rawdev_get_attr(dev_id: u16, attr_name: *const c_char, attr_value: *mut u64) -> c_int;
	pub fn rte_rawdev_get_dev_id(name: *const c_char) -> u16;
	pub fn rte_rawdev_info_get(dev_id: u16, dev_info: *mut rte_rawdev_info) -> c_int;
	pub fn rte_rawdev_pmd_allocate(name: *const c_char, dev_private_size: usize, socket_id: c_int) -> *mut rte_rawdev;
	pub fn rte_rawdev_pmd_init(name: *const c_char, dev_private_size: usize, socket_id: c_int) -> *mut rte_rawdev;
	pub fn rte_rawdev_pmd_release(rawdev: *mut rte_rawdev) -> c_int;
	pub fn rte_rawdev_pmd_uninit(name: *const c_char) -> c_int;
	pub fn rte_rawdev_queue_conf_get(dev_id: u16, queue_id: u16, queue_conf: rte_rawdev_obj_t) -> c_int;
	pub fn rte_rawdev_queue_count(dev_id: u16) -> u16;
	pub fn rte_rawdev_queue_release(dev_id: u16, queue_id: u16) -> c_int;
	pub fn rte_rawdev_queue_setup(dev_id: u16, queue_id: u16, queue_conf: rte_rawdev_obj_t) -> c_int;
	pub fn rte_rawdev_reset(dev_id: u16) -> c_int;
	pub fn rte_rawdev_selftest(dev_id: u16) -> c_int;
	pub fn rte_rawdev_set_attr(dev_id: u16, attr_name: *const c_char, attr_value: u64) -> c_int;
	pub fn rte_rawdev_socket_id(dev_id: u16) -> c_int;
	pub fn rte_rawdev_start(dev_id: u16) -> c_int;
	pub fn rte_rawdev_stop(dev_id: u16);
	pub fn rte_rawdev_xstats_by_name_get(dev_id: u16, name: *const c_char, id: *mut c_uint) -> u64;
	pub fn rte_rawdev_xstats_get(dev_id: u16, ids: *const c_uint, values: *mut u64, n: c_uint) -> c_int;
	pub fn rte_rawdev_xstats_names_get(dev_id: u16, xstats_names: *mut rte_rawdev_xstats_name, size: c_uint) -> c_int;
	pub fn rte_rawdev_xstats_reset(dev_id: u16, ids: *const u32, nb_ids: u32) -> c_int;
}
