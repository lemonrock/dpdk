// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_lpm
{
	pub name: [c_char; 32usize],
	pub max_rules: u32,
	pub number_tbl8s: u32,
	pub rule_info: [rte_lpm_rule_info; 32usize],
	pub __bindgen_padding_0: [u32; 6usize],
	pub tbl24: [rte_lpm_tbl_entry; 16777216usize],
	pub tbl8: *mut rte_lpm_tbl_entry,
	pub rules_tbl: *mut rte_lpm_rule,
	pub __bindgen_padding_1: [u64; 6usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_lpm
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_lpm
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_lpm {{ name: [{}], rule_info: [{}], tbl24: [{}], tbl8: {:?}, rules_tbl: {:?} }}",
			self.name
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
				.collect::<String>(),
			self.rule_info
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
				.collect::<String>(),
			self.tbl24
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
				.collect::<String>(),
			self.tbl8,
			self.rules_tbl
		)
	}
}
