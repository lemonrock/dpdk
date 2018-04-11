// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_intr_allow_others(intr_handle: *mut rte_intr_handle) -> c_int;
	pub fn rte_intr_callback_register(intr_handle: *const rte_intr_handle, cb: rte_intr_callback_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_intr_callback_unregister(intr_handle: *const rte_intr_handle, cb: rte_intr_callback_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_intr_cap_multiple(intr_handle: *mut rte_intr_handle) -> c_int;
	pub fn rte_intr_disable(intr_handle: *const rte_intr_handle) -> c_int;
	pub fn rte_intr_dp_is_en(intr_handle: *mut rte_intr_handle) -> c_int;
	pub fn rte_intr_efd_disable(intr_handle: *mut rte_intr_handle);
	pub fn rte_intr_efd_enable(intr_handle: *mut rte_intr_handle, nb_efd: u32) -> c_int;
	pub fn rte_intr_enable(intr_handle: *const rte_intr_handle) -> c_int;
	pub fn rte_intr_free_epoll_fd(intr_handle: *mut rte_intr_handle);
	pub fn rte_intr_rx_ctl(intr_handle: *mut rte_intr_handle, epfd: c_int, op: c_int, vec: c_uint, data: *mut c_void) -> c_int;
	pub fn rte_intr_tls_epfd() -> c_int;
}
