// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_memzone_dump(f: *mut FILE);
	pub fn rte_memzone_free(mz: *const rte_memzone) -> c_int;
	pub fn rte_memzone_lookup(name: *const c_char) -> *const rte_memzone;
	pub fn rte_memzone_reserve(name: *const c_char, len: usize, socket_id: c_int, flags: c_uint) -> *const rte_memzone;
	pub fn rte_memzone_reserve_aligned(name: *const c_char, len: usize, socket_id: c_int, flags: c_uint, align: c_uint) -> *const rte_memzone;
	pub fn rte_memzone_reserve_bounded(name: *const c_char, len: usize, socket_id: c_int, flags: c_uint, align: c_uint, bound: c_uint) -> *const rte_memzone;
	pub fn rte_memzone_walk(func: Option<unsafe extern "C" fn(arg1: *const rte_memzone, arg: *mut c_void)>, arg: *mut c_void);
}
