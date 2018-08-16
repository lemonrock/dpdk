// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Applies a 'color' to an (IP) packet (optionally taking into account the DSCP / DiffServ field - we pass a table of 64 colors, one for each diffserv value).
/// It can then (with the policer) drop the packet or adjust the color.
///
/// The final color is supposed to be put into 'rte_mbuf.sched.color' according to the documentation, but this doesn't exist.
/// The initial table can be updated over the life of the meter, allowing dynamic policy changes.
///
/// The referenced meter identifier needs to have already been created...
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct MeterReference
{
	/// Meter identifier.
	///
	/// The identified meter needs to have already been created before creating a `METER` action.
	pub identifier: u32,
}

impl MeterReference
{
	#[inline(always)]
	pub(crate) fn rte_flow_action(&self, drop_prevention: &mut Vec<Box<Any>>) -> rte_flow_action
	{
		use self::rte_flow_action_type::*;
		
		rte_flow_action
		{
			type_: RTE_FLOW_ACTION_TYPE_METER,
			conf: box_configuration
			(
				drop_prevention,
				rte_flow_action_meter
				{
					mtr_id: self.identifier,
				}
			),
		}
	}
}
