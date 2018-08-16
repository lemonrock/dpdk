// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Applies the `QUEUE` action for each entry.
///
/// * If there is one entry, then the default receive (RX) or transmit (TX) queue for the packet will be changed.
/// * If more than queue is specified, then the packet will be mirrored; a DPDK driver may not support this.
///
/// * For ingress rules, the values are the zero-based indices of receive (RX) queues.
/// * For egress rules, the values are the zero-based indices of transmit (TX) queues.
/// * For bidirectional rules, we have no knowledge of what could happen.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct QueueActions(BTreeSet<ReceiveQueueIdentifier>);

impl QueueActions
{
	#[inline(always)]
	pub(crate) fn len(&self) -> usize
	{
		self.0.len()
	}
	
	#[inline(always)]
	pub(crate) fn rte_flow_actions(&self, ethernet_device_capabilities: &EthernetDeviceCapabilities, drop_prevention: &mut Vec<Box<Any>>, actions: &mut Vec<rte_flow_action>)
	{
		for receive_queue_identifier in self.0.iter()
		{
			let receive_queue_identifier = *receive_queue_identifier;
			assert!(ethernet_device_capabilities.is_receive_queue_identifier_supported(receive_queue_identifier), "Receive queue identifier '{}' for ethernet device is too large for a QUEUE action", receive_queue_identifier);
			
			actions.push
			(
				rte_flow_action
				{
					type_: rte_flow_action_type::RTE_FLOW_ACTION_TYPE_QUEUE,
					conf: box_configuration
					(
						drop_prevention,
						rte_flow_action_queue
						{
							index: (receive_queue_identifier).into(),
						}
					)
				}
			)
		}
	}
}
