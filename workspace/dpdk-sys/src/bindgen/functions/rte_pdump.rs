// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_pdump_disable(port: u16, queue: u16, flags: u32) -> c_int;
	pub fn rte_pdump_disable_by_deviceid(device_id: *mut c_char, queue: u16, flags: u32) -> c_int;
	pub fn rte_pdump_enable(port: u16, queue: u16, flags: u32, ring: *mut rte_ring, mp: *mut rte_mempool, filter: *mut c_void) -> c_int;
	pub fn rte_pdump_enable_by_deviceid(device_id: *mut c_char, queue: u16, flags: u32, ring: *mut rte_ring, mp: *mut rte_mempool, filter: *mut c_void) -> c_int;
	pub fn rte_pdump_init(path: *const c_char) -> c_int;
	pub fn rte_pdump_set_socket_dir(path: *const c_char, type_: rte_pdump_socktype) -> c_int;
	pub fn rte_pdump_uninit() -> c_int;
}
