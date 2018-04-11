// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_tm_capabilities
{
	pub n_nodes_max: u32,
	pub n_levels_max: u32,
	pub non_leaf_nodes_identical: c_int,
	pub leaf_nodes_identical: c_int,
	pub shaper_n_max: u32,
	pub shaper_private_n_max: u32,
	pub shaper_private_dual_rate_n_max: c_int,
	pub shaper_private_rate_min: u64,
	pub shaper_private_rate_max: u64,
	pub shaper_shared_n_max: u32,
	pub shaper_shared_n_nodes_per_shaper_max: u32,
	pub shaper_shared_n_shapers_per_node_max: u32,
	pub shaper_shared_dual_rate_n_max: u32,
	pub shaper_shared_rate_min: u64,
	pub shaper_shared_rate_max: u64,
	pub shaper_pkt_length_adjust_min: c_int,
	pub shaper_pkt_length_adjust_max: c_int,
	pub sched_n_children_max: u32,
	pub sched_sp_n_priorities_max: u32,
	pub sched_wfq_n_children_per_group_max: u32,
	pub sched_wfq_n_groups_max: u32,
	pub sched_wfq_weight_max: u32,
	pub cman_head_drop_supported: c_int,
	pub cman_wred_context_n_max: u32,
	pub cman_wred_context_private_n_max: u32,
	pub cman_wred_context_shared_n_max: u32,
	pub cman_wred_context_shared_n_nodes_per_context_max: u32,
	pub cman_wred_context_shared_n_contexts_per_node_max: u32,
	pub mark_vlan_dei_supported: [c_int; 3usize],
	pub mark_ip_ecn_tcp_supported: [c_int; 3usize],
	pub mark_ip_ecn_sctp_supported: [c_int; 3usize],
	pub mark_ip_dscp_supported: [c_int; 3usize],
	pub dynamic_update_mask: u64,
	pub stats_mask: u64,
}

impl Default for rte_tm_capabilities
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_tm_capabilities
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_tm_capabilities {{ mark_vlan_dei_supported: {:?}, mark_ip_ecn_tcp_supported: {:?}, mark_ip_ecn_sctp_supported: {:?}, mark_ip_dscp_supported: {:?} }}", self.mark_vlan_dei_supported, self.mark_ip_ecn_tcp_supported, self.mark_ip_ecn_sctp_supported, self.mark_ip_dscp_supported)
	}
}
