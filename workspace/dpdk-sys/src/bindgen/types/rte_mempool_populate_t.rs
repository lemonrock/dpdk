// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub type rte_mempool_populate_t = Option<unsafe extern "C" fn(mp: *mut rte_mempool, max_objs: c_uint, vaddr: *mut c_void, iova: rte_iova_t, len: usize, obj_cb: rte_mempool_populate_obj_cb_t, obj_cb_arg: *mut c_void) -> c_int>;
