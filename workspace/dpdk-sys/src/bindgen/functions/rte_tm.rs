// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_tm_capabilities_get(port_id: u16, cap: *mut rte_tm_capabilities, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_get_number_of_leaf_nodes(port_id: u16, n_leaf_nodes: *mut u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_hierarchy_commit(port_id: u16, clear_on_fail: c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_level_capabilities_get(port_id: u16, level_id: u32, cap: *mut rte_tm_level_capabilities, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_mark_ip_dscp(port_id: u16, mark_green: c_int, mark_yellow: c_int, mark_red: c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_mark_ip_ecn(port_id: u16, mark_green: c_int, mark_yellow: c_int, mark_red: c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_mark_vlan_dei(port_id: u16, mark_green: c_int, mark_yellow: c_int, mark_red: c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_add(port_id: u16, node_id: u32, parent_node_id: u32, priority: u32, weight: u32, level_id: u32, params: *mut rte_tm_node_params, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_capabilities_get(port_id: u16, node_id: u32, cap: *mut rte_tm_node_capabilities, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_cman_update(port_id: u16, node_id: u32, cman: rte_tm_cman_mode, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_delete(port_id: u16, node_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_parent_update(port_id: u16, node_id: u32, parent_node_id: u32, priority: u32, weight: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_resume(port_id: u16, node_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_shaper_update(port_id: u16, node_id: u32, shaper_profile_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_shared_shaper_update(port_id: u16, node_id: u32, shared_shaper_id: u32, add: c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_shared_wred_context_update(port_id: u16, node_id: u32, shared_wred_context_id: u32, add: c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_stats_read(port_id: u16, node_id: u32, stats: *mut rte_tm_node_stats, stats_mask: *mut u64, clear: c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_stats_update(port_id: u16, node_id: u32, stats_mask: u64, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_suspend(port_id: u16, node_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_type_get(port_id: u16, node_id: u32, is_leaf: *mut c_int, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_wfq_weight_mode_update(port_id: u16, node_id: u32, wfq_weight_mode: *mut c_int, n_sp_priorities: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_node_wred_context_update(port_id: u16, node_id: u32, wred_profile_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_ops_get(port_id: u16, error: *mut rte_tm_error) -> *const rte_tm_ops;
	pub fn rte_tm_shaper_profile_add(port_id: u16, shaper_profile_id: u32, profile: *mut rte_tm_shaper_params, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_shaper_profile_delete(port_id: u16, shaper_profile_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_shared_shaper_add_update(port_id: u16, shared_shaper_id: u32, shaper_profile_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_shared_shaper_delete(port_id: u16, shared_shaper_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_shared_wred_context_add_update(port_id: u16, shared_wred_context_id: u32, wred_profile_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_shared_wred_context_delete(port_id: u16, shared_wred_context_id: u32, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_wred_profile_add(port_id: u16, wred_profile_id: u32, profile: *mut rte_tm_wred_params, error: *mut rte_tm_error) -> c_int;
	pub fn rte_tm_wred_profile_delete(port_id: u16, wred_profile_id: u32, error: *mut rte_tm_error) -> c_int;
}
