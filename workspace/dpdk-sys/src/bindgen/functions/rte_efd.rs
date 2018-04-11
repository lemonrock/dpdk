// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_efd_create(name: *const c_char, max_num_rules: u32, key_len: u32, online_cpu_socket_bitmask: u8, offline_cpu_socket: u8) -> *mut rte_efd_table;
	pub fn rte_efd_delete(table: *mut rte_efd_table, socket_id: c_uint, key: *const c_void, prev_value: *mut efd_value_t) -> c_int;
	pub fn rte_efd_find_existing(name: *const c_char) -> *mut rte_efd_table;
	pub fn rte_efd_free(table: *mut rte_efd_table);
	pub fn rte_efd_lookup(table: *const rte_efd_table, socket_id: c_uint, key: *const c_void) -> efd_value_t;
	pub fn rte_efd_lookup_bulk(table: *const rte_efd_table, socket_id: c_uint, num_keys: c_int, key_list: *mut *const c_void, value_list: *mut efd_value_t);
	pub fn rte_efd_update(table: *mut rte_efd_table, socket_id: c_uint, key: *const c_void, value: efd_value_t) -> c_int;
}
