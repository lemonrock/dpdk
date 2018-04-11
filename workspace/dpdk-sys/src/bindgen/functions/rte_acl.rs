// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_acl_add_rules(ctx: *mut rte_acl_ctx, rules: *const rte_acl_rule, num: u32) -> c_int;
	pub fn rte_acl_build(ctx: *mut rte_acl_ctx, cfg: *const rte_acl_config) -> c_int;
	pub fn rte_acl_classify(ctx: *const rte_acl_ctx, data: *mut *const u8, results: *mut u32, num: u32, categories: u32) -> c_int;
	pub fn rte_acl_classify_alg(ctx: *const rte_acl_ctx, data: *mut *const u8, results: *mut u32, num: u32, categories: u32, alg: rte_acl_classify_alg) -> c_int;
	pub fn rte_acl_create(param: *const rte_acl_param) -> *mut rte_acl_ctx;
	pub fn rte_acl_dump(ctx: *const rte_acl_ctx);
	pub fn rte_acl_find_existing(name: *const c_char) -> *mut rte_acl_ctx;
	pub fn rte_acl_free(ctx: *mut rte_acl_ctx);
	pub fn rte_acl_list_dump();
	pub fn rte_acl_reset(ctx: *mut rte_acl_ctx);
	pub fn rte_acl_reset_rules(ctx: *mut rte_acl_ctx);
	pub fn rte_acl_set_ctx_classify(ctx: *mut rte_acl_ctx, alg: rte_acl_classify_alg) -> c_int;
}
