// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_jobstats_abort(job: *mut rte_jobstats) -> c_int;
	pub fn rte_jobstats_context_finish(ctx: *mut rte_jobstats_context);
	pub fn rte_jobstats_context_init(ctx: *mut rte_jobstats_context) -> c_int;
	pub fn rte_jobstats_context_reset(ctx: *mut rte_jobstats_context);
	pub fn rte_jobstats_context_start(ctx: *mut rte_jobstats_context);
	pub fn rte_jobstats_finish(job: *mut rte_jobstats, job_value: i64) -> c_int;
	pub fn rte_jobstats_init(job: *mut rte_jobstats, name: *const c_char, min_period: u64, max_period: u64, initial_period: u64, target: i64) -> c_int;
	pub fn rte_jobstats_reset(job: *mut rte_jobstats);
	pub fn rte_jobstats_set_max(job: *mut rte_jobstats, period: u64);
	pub fn rte_jobstats_set_min(job: *mut rte_jobstats, period: u64);
	pub fn rte_jobstats_set_period(job: *mut rte_jobstats, period: u64, saturate: u8);
	pub fn rte_jobstats_set_target(job: *mut rte_jobstats, target: i64);
	pub fn rte_jobstats_set_update_period_function(job: *mut rte_jobstats, update_period_cb: rte_job_update_period_cb_t);
	pub fn rte_jobstats_start(ctx: *mut rte_jobstats_context, job: *mut rte_jobstats) -> c_int;
}
