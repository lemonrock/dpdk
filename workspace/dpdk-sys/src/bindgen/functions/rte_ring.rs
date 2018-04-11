// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_ring_create(name: *const c_char, count: c_uint, socket_id: c_int, flags: c_uint) -> *mut rte_ring;
	pub fn rte_ring_dump(f: *mut FILE, r: *const rte_ring);
	pub fn rte_ring_free(r: *mut rte_ring);
	pub fn rte_ring_get_memsize(count: c_uint) -> isize;
	pub fn rte_ring_init(r: *mut rte_ring, name: *const c_char, count: c_uint, flags: c_uint) -> c_int;
	pub fn rte_ring_list_dump(f: *mut FILE);
	pub fn rte_ring_lookup(name: *const c_char) -> *mut rte_ring;
}
