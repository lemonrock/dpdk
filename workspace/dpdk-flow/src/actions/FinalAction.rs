// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// The final (last, terminal) action applied to a set of actions associated with a flow rule.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum FinalAction
{
	/// Keeps the packet.
	Keep,
	
	/// Applies the `DROP` action.
	///
	/// Packets are dropped.
	Drop,
	
	/// Applies the `PASSTHRU` action.
	///
	/// Packets are passed to the (?next most important based on priority?) flow rule within the same group.
	PassThrough,
	
	/// Applies the `JUMP` action.
	///
	/// Packets are redirected to another `group` of flow rules.
	Jump
	{
		/// Priority group of rules to redirect to.
		priority_group: FlowRulePriorityGroup,
	},
	
	/// Redirects to the underlying physical function of the device.
	///
	/// Presumably only appropriate for egress rules.
	RedirectToPhysicalFunction,
	
	/// Redirects to the underlying virtual function of the device.
	///
	/// Presumably only appropriate for egress rules.
	RedirectToVirtualFunction
	{
		/// Use original identifier.
		use_original_virtual_function_identifier: bool,
		
		/// Identifier.
		virtual_function_identifier: u32,
	},
	
	/// Redirects to a physical port on a device (eg on a 4-port card, one of ports 0 to 3).
	///
	/// Presumably only appropriate for egress rules.
	RedirectToPhysicalPort
	{
		/// Use original identifier.
		use_original_physical_port_identifier: bool,
		
		/// Identifier.
		physical_port_identifier: u32,
	},
	
	/// Redirects to a DPDK port identifier (`port_id`).
	///
	/// Presumably only appropriate for egress rules.
	RedirectToEthernetPortIdentifierPort
	{
		/// Use original identifier.
		use_original_ethernet_port_identifier: bool,
		
		/// Identifier.
		ethernet_port_identifier: EthernetPortIdentifier,
	},
}

impl Default for FinalAction
{
	#[inline(always)]
	fn default() -> Self
	{
		FinalAction::Keep
	}
}

impl FinalAction
{
	#[inline(always)]
	pub(crate) fn rte_flow_actions(&self, drop_prevention: &mut Vec<Box<Any>>, actions: &mut Vec<rte_flow_action>)
	{
		use self::rte_flow_action_type::*;
		use self::FinalAction::*;
		
		#[inline(always)]
		fn original(use_original: bool) -> u32
		{
			if use_original
			{
				1
			}
			else
			{
				0
			}
		}
		
		const Reserved: u32 = 0;
		
		let (type_, conf) = match *self
		{
			Keep => return,
			
			Drop => (RTE_FLOW_ACTION_TYPE_DROP, null()),
			
			PassThrough => (RTE_FLOW_ACTION_TYPE_PASSTHRU, null()),
			
			Jump { priority_group } => (RTE_FLOW_ACTION_TYPE_JUMP, box_configuration(drop_prevention, rte_flow_action_jump { group: priority_group } )),
			
			RedirectToPhysicalFunction => (RTE_FLOW_ACTION_TYPE_PF, null()),
			
			RedirectToVirtualFunction { use_original_virtual_function_identifier, virtual_function_identifier } => (RTE_FLOW_ACTION_TYPE_VF, box_configuration(drop_prevention,
				rte_flow_action_vf
				{
					bitfield_1: rte_flow_action_vf::newbitfield_1(original(use_original_virtual_function_identifier), Reserved),
					id: virtual_function_identifier,
				}
			)),
			
			RedirectToPhysicalPort { use_original_physical_port_identifier, physical_port_identifier } => (RTE_FLOW_ACTION_TYPE_PHY_PORT, box_configuration(drop_prevention,
				rte_flow_action_phy_port
				{
					bitfield_1: rte_flow_action_phy_port::newbitfield_1(original(use_original_physical_port_identifier), Reserved),
					index: physical_port_identifier,
				}
			)),
			
			RedirectToEthernetPortIdentifierPort { use_original_ethernet_port_identifier, ethernet_port_identifier } => (RTE_FLOW_ACTION_TYPE_PORT_ID, box_configuration(drop_prevention,
				rte_flow_action_port_id
				{
					bitfield_1: rte_flow_action_port_id::newbitfield_1(original(use_original_ethernet_port_identifier), Reserved),
					id: ethernet_port_identifier.into(),
				}
			)),
		};
		
		actions.push
		(
			rte_flow_action
			{
				type_,
				conf,
			}
		)
	}
}
