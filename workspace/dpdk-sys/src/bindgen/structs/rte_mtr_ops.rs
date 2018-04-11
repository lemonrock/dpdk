// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_mtr_ops
{
	pub capabilities_get: rte_mtr_capabilities_get_t,
	pub meter_profile_add: rte_mtr_meter_profile_add_t,
	pub meter_profile_delete: rte_mtr_meter_profile_delete_t,
	pub create: rte_mtr_create_t,
	pub destroy: rte_mtr_destroy_t,
	pub meter_enable: rte_mtr_meter_enable_t,
	pub meter_disable: rte_mtr_meter_disable_t,
	pub meter_profile_update: rte_mtr_meter_profile_update_t,
	pub meter_dscp_table_update: rte_mtr_meter_dscp_table_update_t,
	pub policer_actions_update: rte_mtr_policer_actions_update_t,
	pub stats_update: rte_mtr_stats_update_t,
	pub stats_read: rte_mtr_stats_read_t,
}

impl Default for rte_mtr_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mtr_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mtr_ops {{  }}")
	}
}
