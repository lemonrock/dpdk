// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Flow rule.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct FlowRule
{
	/// Priority group.
	pub priority_group: FlowRulePriorityGroup,
	
	/// Priority level within the above priority group.
	pub priority_level_within_priority_group: u32,

	/// Traffic direction.
	pub traffic_direction: TrafficDirection,

	/// Instead of simply matching the properties of traffic as it would appear on a given DPDK port identifier (`port_id`, port ID), enabling this attribute transfers a flow rule to the lowest possible level of any device endpoints found in the pattern.
	///
	/// When supported, this effectively enables an application to re-route traffic not necessarily intended for it (eg coming from or addressed to different physical ports, virtual functions (VFs) or applications) at the device level.
	///
	/// It complements the behavior of some pattern items such as `Pattern::PhysicalPort` and is meaningless without them.
	///
	/// When transferring flow rules, ingress and egress attributes keep their original meaning, as if processing traffic emitted or received by the application.
	pub transfer: bool,
	
	/// Patterns.
	pub patterns: ArrayVec<[Pattern; Pattern::MaximumPatterns]>,
}

impl FlowRule
{
	/// Creates an active flow rule.
	#[inline(always)]
	pub fn activate(&self, port_identifier: u16) -> Result<ActiveFlowRule, rte_flow_error>
	{
		let attributes = self.create_flow_attributes();
		let mut error = unsafe { zeroed() };
		
		let patterns = Pattern::rte_flow_items(&self.patterns);
		
		let actions: ArrayVec<[rte_flow_action; 16]> = ArrayVec::new();
		
		let result = unsafe { rte_flow_create(port_identifier, &attributes, patterns.as_ptr(), actions.as_ptr(), &mut error) };
		
		if likely!(result.is_null())
		{
			Ok
			(
				ActiveFlowRule
				{
					port_identifier,
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
				
				NegativeE::EIO => panic!("underlying deevice '{}' is removed", port_identifier),
				NegativeE::EINVAL => panic!("unknown or invalid rule specification"),
				NegativeE::EEXIST => panic!("collision with an existing rule. Only returned if device supports flow rule collision checking and there was a flow rule collision. Not receiving this return code is no guarantee that creating the rule will not fail due to a collision"),
				NegativeE::EBUSY => panic!("action cannot be performed due to busy device resources, may succeed if the affected queues or even the entire port are in a stopped state (see `rte_eth_dev_rx_queue_stop()` and `rte_eth_dev_stop()`)"),
				
				unknown @ _ => panic!("unknown error code '{}' from rte_flow_create, extra data was '{:?}'", unknown, error),
			}
		}
	}
	
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




pub enum FinalAction
{
	/// Applies the `PASSTHRU` action.
	///
	/// Packets are passed to the (?next most important based on priority?) flow rule within the same group.
	PassThrough,

	/// Applies the `DROP` action.
	///
	/// Packets are dropped.
	Drop,
	
	/// Applies the `JUMP` action.
	///
	/// Packets are redirected to another `group` of flow rules.
	Jump
	{
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
		#[allow(missing(docs))]
		use_original_virtual_function_identifier: bool,

		#[allow(missing(docs))]
		virtual_function_identifier: u32,
	},

	/// Redirects to a physical port on a device (eg on a 4-port card, one of ports 0 to 3).
	///
	/// Presumably only appropriate for egress rules.
	RedirectToPhysicalPort
	{
		#[allow(missing(docs))]
		use_original_physical_port_identifier: bool,

		#[allow(missing(docs))]
		physical_port_identifier: u32,
	},

	/// Redirects to a DPDK port identifier (`port_id`).
	///
	/// Presumably only appropriate for egress rules.
	RedirectToPortIdentifierPort
	{
		#[allow(missing(docs))]
		use_original_port_identifier: bool,

		#[allow(missing(docs))]
		port_identifier: u16,
	},
}

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
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct FlowAction
{
	/// Applies the `METER` action.
	#[serde(default)]
	pub meter: Option<Meter>,
	
	/// Applies the `COUNT` action for each entry.
	#[serde(default)]
	pub counters: BTreeMap<CounterIdentifier, CounterSharing>,
	
	/// Apply the `FLAG` action.
	///
	/// * Sets the packet buffer (`rte_mbuf`)'s offload feature bit flag `PKT_RX_FDIR` in the bit flags field `rte_mbuf.ol_flags`.
	#[serde(default)]
	pub flag: bool,
	
	/// Apply the `MARK` action with this value if not `None`.
	///
	/// Each DPDK driver has a limited range of supported values, which could be as small as 0 and 1.
	///
	/// * Provides a value which can be matched in other flow rules with the `Pattern::Mark` pattern.
	/// * Sets the packet buffer (`rte_mbuf`)'s offload feature bit flags `PKT_RX_FDIR` and `PKT_RX_FDIR_ID`, in the bit flags field `rte_mbuf.ol_flags`.
	/// * Sets the packet buffer (`rte_mbuf`)'s union field `rte_mbuf.hash.fdir.hi` to the value of the mark.
	///   * This value can co-exist with `rte_mbuf.hash.rss`, which is equivalent to `rte_mbuf.hash.fdir.lo`.
	///   * This value can be used by the `dpdk-packet-distributor` crate (via `rte_mbuf.hash.usr`).
	#[serde(default)]
	pub mark: Option<u32>,
	
	/// Applies the `QUEUE` action for each entry.
	///
	/// * If there is one entry, then the default receive (RX) or transmit (TX) queue for the packet will be changed.
	/// * If more than queue is specified, then the packet will be mirrored; a DPDK driver may not support this.
	///
	/// * For ingress rules, the values are the zero-based indices of receive (RX) queues.
	/// * For egress rules, the values are the zero-based indices of transmit (TX) queues.
	/// * For bidirectional rules, we have no knowledge of what could happen.
	#[serde(default)]
	pub queues: BTreeSet<u16>,

	/// Calculates a Receive Side Scaling (RSS) hash value, and sends the packet to the appropriate receive (RX) queue for that hash value.
	///
	/// Presumably is only appropriate for ingress rules; the DPDK documentation is silent on the matter.
	///
	/// * Sets the packet buffer (`rte_mbuf`)'s union field `rte_mbuf.hash.rss` to the value of the calculated hash.
	///   * This value is equivalent to `rte_mbuf.hash.fdir.lo` and so can exist with `mark`, which is set in `rte_mbuf.hash.fdir.hi` (see above).
	/// * Can be used in conjunction with `queue`.
	#[serde(default)]
	pub receive_side_scaling: Option<ReceiveSideScaling>,
}
	// Terminal: JUMP, DROP, PASSTHRU, PF, VF, PHY_PORT, PORT_ID
	/*
	
	
}
