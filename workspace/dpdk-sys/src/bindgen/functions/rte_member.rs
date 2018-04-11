// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_member_add(setsum: *const rte_member_setsum, key: *const c_void, set_id: member_set_t) -> c_int;
	pub fn rte_member_create(params: *const rte_member_parameters) -> *mut rte_member_setsum;
	pub fn rte_member_delete(setsum: *const rte_member_setsum, key: *const c_void, set_id: member_set_t) -> c_int;
	pub fn rte_member_find_existing(name: *const c_char) -> *mut rte_member_setsum;
	pub fn rte_member_free(setsum: *mut rte_member_setsum);
	pub fn rte_member_lookup(setsum: *const rte_member_setsum, key: *const c_void, set_id: *mut member_set_t) -> c_int;
	pub fn rte_member_lookup_bulk(setsum: *const rte_member_setsum, keys: *mut *const c_void, num_keys: u32, set_ids: *mut member_set_t) -> c_int;
	pub fn rte_member_lookup_multi(setsum: *const rte_member_setsum, key: *const c_void, max_match_per_key: u32, set_id: *mut member_set_t) -> c_int;
	pub fn rte_member_lookup_multi_bulk(setsum: *const rte_member_setsum, keys: *mut *const c_void, num_keys: u32, max_match_per_key: u32, match_count: *mut u32, set_ids: *mut member_set_t) -> c_int;
	pub fn rte_member_reset(setsum: *const rte_member_setsum);
}
