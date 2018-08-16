// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// The actions to take when a flow rule matches.
///
/// The following DPDK actions are not supported and no implementation is planned:-
///
/// * `VOID` (as it is of no benefit in this API).
/// * `SECURITY` (related to hardware offloading of IPSec and crypograhic algorithms).
/// * `OF_SET_MPLS_TTL` (sets the MPLS hops count).
/// * `OF_DEC_MPLS_TTL` (decrements the MPLS hops count).
/// * `OF_SET_NW_TTL` (sets the Internet Protocol hops count).
/// * `OF_DEC_NW_TTL` (decrements the Internet Protocol hops count).
/// * `OF_COPY_TTL_OUT`
/// * `OF_COPY_TTL_IN`
/// * `OF_POP_VLAN`
/// * `OF_PUSH_VLAN`
/// * `OF_SET_VLAN_VID` (sets the IEEE 802.1q Virtual LAN identifier).
/// * `OF_SET_VLAN_PCP` (sets the IEEE 802.1q Virtual LAN priority).
/// * `OF_POP_MPLS`
/// * `OF_PUSH_MPLS`
/// * `VXLAN_ENCAP`
/// * `VXLAN_DECAP`
/// * `NVGRE_ENCAP`
/// * `NVGRE_DECAP`
///
/// Note that the `END` DPDK action is applied automatically.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct FlowActions
{
	/// Applies the `METER` action.
	#[serde(default)]
	pub meter: Option<MeterReference>,
	
	/// Applies the `COUNT` actions
	#[serde(default)]
	pub counters: CounterActions,
	
	/// Tag this packet using a flag or a mark.
	#[serde(default)]
	pub tag: Option<TagAction>,
	
	/// Applies the `QUEUE` actions.
	#[serde(default)]
	pub queues: QueueActions,

	/// Calculates a Receive Side Scaling (RSS) hash value, and sends the packet to the appropriate receive (RX) queue for that hash value.
	///
	/// Presumably is only appropriate for ingress rules; the DPDK documentation is silent on the matter.
	///
	/// * Sets the packet buffer (`rte_mbuf`)'s union field `rte_mbuf.hash.rss` to the value of the calculated hash.
	///   * This value is equivalent to `rte_mbuf.hash.fdir.lo` and so can exist with `mark`, which is set in `rte_mbuf.hash.fdir.hi` (see above).
	/// * Can be used in conjunction with `queue`.
	#[serde(default)]
	pub receive_side_scaling: Option<Box<ReceiveSideScalingAction>>,
	
	/// Final action to take.
	#[serde(default)]
	pub final_action: FinalAction,
}

impl FlowActions
{
	pub(crate) fn rte_flow_actions(&self, ethernet_device_capabilities: &EthernetDeviceCapabilities, drop_prevention: &mut Vec<Box<Any>>) -> Vec<rte_flow_action>
	{
		use self::rte_flow_action_type::*;
		
		let mut actions = Vec::with_capacity(1 + self.counters.len() + 1 + self.queues.len() + 1 + 1 + 1);
		
		if let Some(ref meter) = self.meter
		{
			actions.push(meter.rte_flow_action(drop_prevention));
		}
		
		self.counters.rte_flow_actions(drop_prevention, &mut actions);
		
		if let Some(ref tag) = self.tag
		{
			actions.push(tag.rte_flow_action(drop_prevention));
		}
		
		self.queues.rte_flow_actions(ethernet_device_capabilities, drop_prevention, &mut actions);
		
		if let Some(ref receive_side_scaling) = self.receive_side_scaling
		{
			actions.push(receive_side_scaling.rte_flow_action(ethernet_device_capabilities, drop_prevention));
		}
		
		self.final_action.rte_flow_actions(drop_prevention, &mut actions);
		
		assert_ne!(actions.len(), 0, "There must be at least one action, and it can not be FinalAction::Keep");
		
		actions.push
		(
			rte_flow_action
			{
				type_: RTE_FLOW_ACTION_TYPE_END,
				conf: null(),
			}
		);
		
		actions
	}
	
	#[inline(always)]
	pub(crate) fn counters_to_hash_map(&self) -> HashMap<CounterIdentifier, CounterSharing>
	{
		self.counters.to_hash_map()
	}
}
