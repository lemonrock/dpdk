// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_kvargs_count(kvlist: *const rte_kvargs, key_match: *const c_char) -> c_uint;
	pub fn rte_kvargs_free(kvlist: *mut rte_kvargs);
	pub fn rte_kvargs_parse(args: *const c_char, valid_keys: *const *const c_char) -> *mut rte_kvargs;
	pub fn rte_kvargs_process(kvlist: *const rte_kvargs, key_match: *const c_char, handler: arg_handler_t, opaque_arg: *mut c_void) -> c_int;
}
