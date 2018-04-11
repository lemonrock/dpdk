// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_latencystats_get(values: *mut rte_metric_value, size: u16) -> c_int;
	pub fn rte_latencystats_get_names(names: *mut rte_metric_name, size: u16) -> c_int;
	pub fn rte_latencystats_init(samp_intvl: u64, user_cb: rte_latency_stats_flow_type_fn) -> c_int;
	pub fn rte_latencystats_uninit() -> c_int;
	pub fn rte_latencystats_update() -> i32;
}
