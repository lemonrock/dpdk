// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_calloc(type_: *const c_char, num: usize, size: usize, align: c_uint) -> *mut c_void;
	pub fn rte_calloc_socket(type_: *const c_char, num: usize, size: usize, align: c_uint, socket: c_int) -> *mut c_void;
	pub fn rte_free(ptr: *mut c_void);
	pub fn rte_malloc(type_: *const c_char, size: usize, align: c_uint) -> *mut c_void;
	pub fn rte_malloc_dump_heaps(f: *mut FILE);
	pub fn rte_malloc_dump_stats(f: *mut FILE, type_: *const c_char);
	pub fn rte_malloc_get_socket_stats(socket: c_int, socket_stats: *mut rte_malloc_socket_stats) -> c_int;
	pub fn rte_malloc_set_limit(type_: *const c_char, max: usize) -> c_int;
	pub fn rte_malloc_socket(type_: *const c_char, size: usize, align: c_uint, socket: c_int) -> *mut c_void;
	pub fn rte_malloc_validate(ptr: *const c_void, size: *mut usize) -> c_int;
	pub fn rte_malloc_virt2iova(addr: *const c_void) -> rte_iova_t;
	pub fn rte_mbuf_best_mempool_ops() -> *const c_char;
	pub fn rte_mbuf_platform_mempool_ops() -> *const c_char;
	pub fn rte_mbuf_sanity_check(m: *const rte_mbuf, is_header: c_int);
	pub fn rte_mbuf_set_platform_mempool_ops(ops_name: *const c_char) -> c_int;
	pub fn rte_mbuf_set_user_mempool_ops(ops_name: *const c_char) -> c_int;
	pub fn rte_mbuf_user_mempool_ops() -> *const c_char;
	pub fn rte_mem_alloc_validator_register(name: *const c_char, clb: rte_mem_alloc_validator_t, socket_id: c_int, limit: usize) -> c_int;
	pub fn rte_mem_alloc_validator_unregister(name: *const c_char, socket_id: c_int) -> c_int;
	pub fn rte_mem_event_callback_register(name: *const c_char, clb: rte_mem_event_callback_t, arg: *mut c_void) -> c_int;
	pub fn rte_mem_event_callback_unregister(name: *const c_char, arg: *mut c_void) -> c_int;
	pub fn rte_mem_iova2virt(iova: rte_iova_t) -> *mut c_void;
	pub fn rte_mem_lock_page(virt: *const c_void) -> c_int;
	pub fn rte_mem_virt2iova(virt: *const c_void) -> rte_iova_t;
	pub fn rte_mem_virt2memseg(virt: *const c_void, msl: *const rte_memseg_list) -> *mut rte_memseg;
	pub fn rte_mem_virt2memseg_list(virt: *const c_void) -> *mut rte_memseg_list;
	pub fn rte_mem_virt2phy(virt: *const c_void) -> phys_addr_t;
	pub fn rte_memdump(f: *mut FILE, title: *const c_char, buf: *const c_void, len: c_uint);
	pub fn rte_memory_get_nchannel() -> c_uint;
	pub fn rte_memory_get_nrank() -> c_uint;
	pub fn rte_realloc(ptr: *mut c_void, size: usize, align: c_uint) -> *mut c_void;
	pub fn rte_zmalloc(type_: *const c_char, size: usize, align: c_uint) -> *mut c_void;
	pub fn rte_zmalloc_socket(type_: *const c_char, size: usize, align: c_uint, socket: c_int) -> *mut c_void;
}
