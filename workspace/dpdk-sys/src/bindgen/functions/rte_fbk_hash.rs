// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_fbk_hash_create(params: *const rte_fbk_hash_params) -> *mut rte_fbk_hash_table;
	pub fn rte_fbk_hash_find_existing(name: *const c_char) -> *mut rte_fbk_hash_table;
	pub fn rte_fbk_hash_free(ht: *mut rte_fbk_hash_table);
}
