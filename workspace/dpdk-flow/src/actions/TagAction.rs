// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Applies a flag or a mark.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum TagAction
{
	/// Apply the `FLAG` action.
	///
	/// * Sets the packet buffer (`rte_mbuf`)'s offload feature bit flag `PKT_RX_FDIR` in the bit flags field `rte_mbuf.ol_flags`.
	Flag,
	
	/// Apply the `MARK` action with this value if not `None`.
	///
	/// Each DPDK driver has a limited range of supported values, which could be as small as 0 and 1.
	///
	/// * Provides a value which can be matched in other flow rules with the `Pattern::Mark` pattern.
	/// * Sets the packet buffer (`rte_mbuf`)'s offload feature bit flags `PKT_RX_FDIR` and `PKT_RX_FDIR_ID`, in the bit flags field `rte_mbuf.ol_flags`.
	/// * Sets the packet buffer (`rte_mbuf`)'s union field `rte_mbuf.hash.fdir.hi` to the value of the mark.
	///   * This value can co-exist with `rte_mbuf.hash.rss`, which is equivalent to `rte_mbuf.hash.fdir.lo`.
	///   * This value can be used by the `dpdk-packet-distributor` crate (via `rte_mbuf.hash.usr`).
	Mark(u32),
}

impl Default for TagAction
{
	#[inline(always)]
	fn default() -> Self
	{
		TagAction::Flag
	}
}

impl TagAction
{
	#[inline(always)]
	pub(crate) fn rte_flow_action(&self, drop_prevention: &mut Vec<Box<Any>>) -> rte_flow_action
	{
		use self::rte_flow_action_type::*;
		use self::TagAction::*;
		
		match *self
		{
			Flag =>
			{
				rte_flow_action
				{
					type_: RTE_FLOW_ACTION_TYPE_FLAG,
					conf: null(),
				}
			}
			
			Mark(mark) =>
			{
				rte_flow_action
				{
					type_: RTE_FLOW_ACTION_TYPE_MARK,
					conf: box_configuration(drop_prevention, rte_flow_action_mark { id: mark }),
				}
			}
		}
	}
}
