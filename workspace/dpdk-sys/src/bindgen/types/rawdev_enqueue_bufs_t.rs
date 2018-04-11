// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub type rawdev_enqueue_bufs_t = Option<unsafe extern "C" fn(dev: *mut rte_rawdev, buffers: *mut *mut rte_rawdev_buf, count: c_uint, context: rte_rawdev_obj_t) -> c_int>;
