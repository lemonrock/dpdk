// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_keepalive_create(callback: rte_keepalive_failure_callback_t, data: *mut c_void) -> *mut rte_keepalive;
	pub fn rte_keepalive_dispatch_pings(ptr_timer: *mut c_void, ptr_data: *mut c_void);
	pub fn rte_keepalive_mark_alive(keepcfg: *mut rte_keepalive);
	pub fn rte_keepalive_mark_sleep(keepcfg: *mut rte_keepalive);
	pub fn rte_keepalive_register_core(keepcfg: *mut rte_keepalive, id_core: c_int);
	pub fn rte_keepalive_register_relay_callback(keepcfg: *mut rte_keepalive, callback: rte_keepalive_relay_callback_t, data: *mut c_void);
}
