// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_pmd_i40e_queue_region_info
{
	pub region_id: u8,
	pub queue_start_index: u8,
	pub queue_num: u8,
	pub user_priority_num: u8,
	pub user_priority: [u8; 8usize],
	pub flowtype_num: u8,
	pub hw_flowtype: [u8; 64usize],
}

impl Default for rte_pmd_i40e_queue_region_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_pmd_i40e_queue_region_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_pmd_i40e_queue_region_info {{ user_priority: {:?}, hw_flowtype: [{}] }}",
			self.user_priority,
			self.hw_flowtype
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>()
		)
	}
}
