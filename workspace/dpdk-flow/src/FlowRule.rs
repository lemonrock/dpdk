// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Flow rule.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct FlowRule
{
	/// Priority group.
	#[serde(default)]
	pub priority_group: FlowRulePriorityGroup,
	
	/// Priority level within the above priority group.
	#[serde(default)]
	pub priority_level_within_priority_group: u32,

	/// Traffic direction.
	#[serde(default)]
	pub traffic_direction: TrafficDirection,

	/// Instead of simply matching the properties of traffic as it would appear on a given DPDK port identifier (`port_id`, port ID), enabling this attribute transfers a flow rule to the lowest possible level of any device endpoints found in the pattern.
	///
	/// When supported, this effectively enables an application to re-route traffic not necessarily intended for it (eg coming from or addressed to different physical ports, virtual functions (VFs) or applications) at the device level.
	///
	/// It complements the behavior of some pattern items such as `Pattern::PhysicalPort` and is meaningless without them.
	///
	/// When transferring flow rules, ingress and egress attributes keep their original meaning, as if processing traffic emitted or received by the application.
	#[serde(default)]
	pub transfer: bool,
	
	/// Patterns.
	#[serde(default)]
	pub patterns: ArrayVec<[Box<Pattern>; Pattern::MaximumPatterns]>,

	/// Actions.
	#[serde(default)]
	pub actions: FlowActions,
}

impl FlowRuleConfiguration for FlowRule
{
	type ActiveFlowRuleHandle = ActiveFlowRule;
	
	/// Creates an active flow rule.
	#[inline(always)]
	fn configure(&self, ethernet_port_identifier: EthernetPortIdentifier, ethernet_device_capabilities: &EthernetDeviceCapabilities) -> Result<Self::ActiveFlowRuleHandle, rte_flow_error>
	{
		let attributes = self.create_flow_attributes();
		let mut error = unsafe { zeroed() };
		
		let mut drop_prevention = Vec::with_capacity(32);
		
		assert_ne!(self.patterns.len(), 0, "There must be at least one pattern");
		
		let patterns = Pattern::rte_flow_items(&self.patterns, &mut drop_prevention);
		
		let actions = self.actions.rte_flow_actions(ethernet_device_capabilities, &mut drop_prevention);
		
		let result = unsafe { rte_flow_create(ethernet_port_identifier.into(), &attributes, patterns.as_ptr(), actions.as_ptr(), &mut error) };
		
		drop(drop_prevention);
		
		if likely!(result.is_null())
		{
			Ok
			(
				ActiveFlowRule
				{
					ethernet_port_identifier,
					reference: unsafe { NonNull::new_unchecked(result) },
				}
			)
		}
		else
		{
			match LogicalCore::current_logical_core_error_number()
			{
				// Underlying device does not support this functionality.
				NegativeE::ENOSYS => Err(error),
				
				// Valid but unsupported rule specification (eg partial bitmasks are unsupported).
				NegativeE::ENOTSUP => Err(error),
				
				// Not enough memory to execute the function, or if the device supports resource validation, resource limitation on the device.
				NegativeE::ENOMEM => Err(error),
				
				NegativeE::EIO => panic!("underlying deevice '{}' is removed", ethernet_port_identifier),
				NegativeE::EINVAL => panic!("unknown or invalid rule specification"),
				NegativeE::EEXIST => panic!("collision with an existing rule. Only returned if device supports flow rule collision checking and there was a flow rule collision. Not receiving this return code is no guarantee that creating the rule will not fail due to a collision"),
				NegativeE::EBUSY => panic!("action cannot be performed due to busy device resources, may succeed if the affected queues or even the entire port are in a stopped state (see `rte_eth_dev_rx_queue_stop()` and `rte_eth_dev_stop()`)"),
				
				unknown @ _ => panic!("unknown error code '{}' from rte_flow_create, extra data was '{:?}'", unknown, error),
			}
		}
	}
}

impl FlowRule
{
	#[inline(always)]
	fn create_flow_attributes(&self) -> rte_flow_attr
	{
		let (ingress, egress) = self.traffic_direction.ingress_and_egress_bits();
		
		let transfer = if self.transfer
		{
			1
		}
		else
		{
			0
		};
		
		const Reserved: u32 = 0;
		
		rte_flow_attr
		{
			group: self.priority_group,
			priority: self.priority_level_within_priority_group,
			bitfield_1: rte_flow_attr::newbitfield_1(ingress, egress, transfer, Reserved),
		}
	}
}
