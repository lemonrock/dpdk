// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_tm_ops
{
	pub node_type_get: rte_tm_node_type_get_t,
	pub capabilities_get: rte_tm_capabilities_get_t,
	pub level_capabilities_get: rte_tm_level_capabilities_get_t,
	pub node_capabilities_get: rte_tm_node_capabilities_get_t,
	pub wred_profile_add: rte_tm_wred_profile_add_t,
	pub wred_profile_delete: rte_tm_wred_profile_delete_t,
	pub shared_wred_context_add_update: rte_tm_shared_wred_context_add_update_t,
	pub shared_wred_context_delete: rte_tm_shared_wred_context_delete_t,
	pub shaper_profile_add: rte_tm_shaper_profile_add_t,
	pub shaper_profile_delete: rte_tm_shaper_profile_delete_t,
	pub shared_shaper_add_update: rte_tm_shared_shaper_add_update_t,
	pub shared_shaper_delete: rte_tm_shared_shaper_delete_t,
	pub node_add: rte_tm_node_add_t,
	pub node_delete: rte_tm_node_delete_t,
	pub node_suspend: rte_tm_node_suspend_t,
	pub node_resume: rte_tm_node_resume_t,
	pub hierarchy_commit: rte_tm_hierarchy_commit_t,
	pub node_parent_update: rte_tm_node_parent_update_t,
	pub node_shaper_update: rte_tm_node_shaper_update_t,
	pub node_shared_shaper_update: rte_tm_node_shared_shaper_update_t,
	pub node_stats_update: rte_tm_node_stats_update_t,
	pub node_wfq_weight_mode_update: rte_tm_node_wfq_weight_mode_update_t,
	pub node_cman_update: rte_tm_node_cman_update_t,
	pub node_wred_context_update: rte_tm_node_wred_context_update_t,
	pub node_shared_wred_context_update: rte_tm_node_shared_wred_context_update_t,
	pub node_stats_read: rte_tm_node_stats_read_t,
	pub mark_vlan_dei: rte_tm_mark_vlan_dei_t,
	pub mark_ip_ecn: rte_tm_mark_ip_ecn_t,
	pub mark_ip_dscp: rte_tm_mark_ip_dscp_t,
}

impl Default for rte_tm_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_tm_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_tm_ops {{  }}")
	}
}
