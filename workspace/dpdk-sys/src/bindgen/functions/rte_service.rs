// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_service_attr_get(id: u32, attr_id: u32, attr_value: *mut u32) -> i32;
	pub fn rte_service_attr_reset_all(id: u32) -> i32;
	pub fn rte_service_component_register(spec: *const rte_service_spec, service_id: *mut u32) -> i32;
	pub fn rte_service_component_runstate_set(id: u32, runstate: u32) -> i32;
	pub fn rte_service_component_unregister(id: u32) -> i32;
	pub fn rte_service_dump(f: *mut FILE, id: u32) -> i32;
	pub fn rte_service_finalize();
	pub fn rte_service_get_by_name(name: *const c_char, service_id: *mut u32) -> i32;
	pub fn rte_service_get_count() -> u32;
	pub fn rte_service_get_name(id: u32) -> *const c_char;
	pub fn rte_service_init() -> i32;
	pub fn rte_service_lcore_add(lcore: u32) -> i32;
	pub fn rte_service_lcore_count() -> i32;
	pub fn rte_service_lcore_count_services(lcore: u32) -> i32;
	pub fn rte_service_lcore_del(lcore: u32) -> i32;
	pub fn rte_service_lcore_list(array: *mut u32, n: u32) -> i32;
	pub fn rte_service_lcore_reset_all() -> i32;
	pub fn rte_service_lcore_start(lcore_id: u32) -> i32;
	pub fn rte_service_lcore_stop(lcore_id: u32) -> i32;
	pub fn rte_service_map_lcore_get(service_id: u32, lcore: u32) -> i32;
	pub fn rte_service_map_lcore_set(service_id: u32, lcore: u32, enable: u32) -> i32;
	pub fn rte_service_probe_capability(id: u32, capability: u32) -> i32;
	pub fn rte_service_run_iter_on_app_lcore(id: u32, serialize_multithread_unsafe: u32) -> i32;
	pub fn rte_service_runstate_get(id: u32) -> i32;
	pub fn rte_service_runstate_set(id: u32, runstate: u32) -> i32;
	pub fn rte_service_set_runstate_mapped_check(id: u32, enable: i32) -> i32;
	pub fn rte_service_set_stats_enable(id: u32, enable: i32) -> i32;
	pub fn rte_service_start_with_defaults() -> i32;
}
