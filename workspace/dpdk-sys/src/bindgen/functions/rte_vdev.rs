// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_vdev_add_custom_scan(callback: rte_vdev_scan_callback, user_arg: *mut c_void) -> c_int;
	pub fn rte_vdev_init(name: *const c_char, args: *const c_char) -> c_int;
	pub fn rte_vdev_register(driver: *mut rte_vdev_driver);
	pub fn rte_vdev_remove_custom_scan(callback: rte_vdev_scan_callback, user_arg: *mut c_void) -> c_int;
	pub fn rte_vdev_uninit(name: *const c_char) -> c_int;
	pub fn rte_vdev_unregister(driver: *mut rte_vdev_driver);
}
