// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_reorder_create(name: *const c_char, socket_id: c_uint, size: c_uint) -> *mut rte_reorder_buffer;
	pub fn rte_reorder_drain(b: *mut rte_reorder_buffer, mbufs: *mut *mut rte_mbuf, max_mbufs: c_uint) -> c_uint;
	pub fn rte_reorder_find_existing(name: *const c_char) -> *mut rte_reorder_buffer;
	pub fn rte_reorder_free(b: *mut rte_reorder_buffer);
	pub fn rte_reorder_init(b: *mut rte_reorder_buffer, bufsize: c_uint, name: *const c_char, size: c_uint) -> *mut rte_reorder_buffer;
	pub fn rte_reorder_insert(b: *mut rte_reorder_buffer, mbuf: *mut rte_mbuf) -> c_int;
	pub fn rte_reorder_reset(b: *mut rte_reorder_buffer);
}
