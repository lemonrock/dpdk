// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rust___mempool_generic_get(mp: *mut rte_mempool, obj_table: *mut *mut c_void, n: c_uint, cache: *mut rte_mempool_cache) -> c_int;
	pub fn rust___mempool_generic_put(mp: *mut rte_mempool, obj_table: *const *mut c_void, n: c_uint, cache: *mut rte_mempool_cache);
	pub fn rust___mempool_get_header(obj: *mut c_void) -> *mut rte_mempool_objhdr;
	pub fn rust___mempool_get_trailer(obj: *mut c_void) -> *mut rte_mempool_objtlr;
}
