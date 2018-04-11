// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_mtr_capabilities_get(port_id: u16, cap: *mut rte_mtr_capabilities, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_create(port_id: u16, mtr_id: u32, params: *mut rte_mtr_params, shared: c_int, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_destroy(port_id: u16, mtr_id: u32, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_meter_disable(port_id: u16, mtr_id: u32, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_meter_dscp_table_update(port_id: u16, mtr_id: u32, dscp_table: *mut rte_mtr_color, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_meter_enable(port_id: u16, mtr_id: u32, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_meter_profile_add(port_id: u16, meter_profile_id: u32, profile: *mut rte_mtr_meter_profile, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_meter_profile_delete(port_id: u16, meter_profile_id: u32, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_meter_profile_update(port_id: u16, mtr_id: u32, meter_profile_id: u32, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_ops_get(port_id: u16, error: *mut rte_mtr_error) -> *const rte_mtr_ops;
	pub fn rte_mtr_policer_actions_update(port_id: u16, mtr_id: u32, action_mask: u32, actions: *mut rte_mtr_policer_action, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_stats_read(port_id: u16, mtr_id: u32, stats: *mut rte_mtr_stats, stats_mask: *mut u64, clear: c_int, error: *mut rte_mtr_error) -> c_int;
	pub fn rte_mtr_stats_update(port_id: u16, mtr_id: u32, stats_mask: u64, error: *mut rte_mtr_error) -> c_int;
}
