// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet matchers.
///
/// The following DPDK matchers are not yet implemented but are planned to be supported:-
///
/// * `RTE_FLOW_ITEM_TYPE_RAW`
/// * `RTE_FLOW_ITEM_TYPE_UDP`
/// * `RTE_FLOW_ITEM_TYPE_TCP`
/// * `RTE_FLOW_ITEM_TYPE_ICMP6_ND_NS`
/// * `RTE_FLOW_ITEM_TYPE_ICMP6_ND_NA`
/// * `RTE_FLOW_ITEM_TYPE_ICMP6_ND_OPT`
/// * `RTE_FLOW_ITEM_TYPE_ICMP6_ND_OPT_SLA_ETH`
/// * `RTE_FLOW_ITEM_TYPE_ICMP6_ND_OPT_TLA_ETH`
///
/// The following DPDK matchers are not yet implemented and there are no plans to support them:-
///
/// * `RTE_FLOW_ITEM_TYPE_SCTP`
/// * `RTE_FLOW_ITEM_TYPE_VXLAN`
/// * `RTE_FLOW_ITEM_TYPE_E_TAG`
/// * `RTE_FLOW_ITEM_TYPE_NVGRE`
/// * `RTE_FLOW_ITEM_TYPE_MPLS`
/// * `RTE_FLOW_ITEM_TYPE_GRE`
/// * `RTE_FLOW_ITEM_TYPE_GTP`
/// * `RTE_FLOW_ITEM_TYPE_GTPC`
/// * `RTE_FLOW_ITEM_TYPE_GTPU`
/// * `RTE_FLOW_ITEM_TYPE_ESP`
/// * `RTE_FLOW_ITEM_TYPE_GENEVE`
/// * `RTE_FLOW_ITEM_TYPE_VXLAN_GPE`
pub enum PacketMatcher
{
	/// A matcher that matches an Address Resolution Protocol (ARP) Internet Protocol (IP) version 4 packet over Ethernet.
	///
	/// The underlying DPDK functionality supports other kinds of ARP headers but always assumes an InternetProtocolVersion4-sized payload!
	AddressResolutionProtocolForInternetProtocolVersion4OverEthernet(MaskedPacketMatcherFields<AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification, AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask>),
	
	/// Matches at a number of layers.
	Any(MaskedPacketMatcherFields<u32, u32>),
	
	/// A matcher that matches an ethernet header.
	///
	/// When followed by a 'layer 2.5' matcher such as VirtualLanHeaderPacketMatcher, the Ether Type is a tag protocol identifier (TPID).
	/// In this case, the ether type refers to the outer header, with the VirtualLanHeaderPacketMatcher's ether type referring to the inner Ether Type or tag protocol identifier (TPID).
	EthernetHeader(MaskedPacketMatcherFields<EthernetHeaderSpecification, EthernetHeaderMask>),
	
	/// Fuzzy pattern match.
	///
	/// Not all devices will support a fuzzy match.
	///
	/// Usually a fuzzy match is fast but the cost is accuracy, eg Signature Match only match pattern's hash value, but it is possible two different patterns have the same hash value.
	///
	/// Matching accuracy level can be configure by a `threshold`.
	///
	/// These are mapped internally by a DPDK driver to the different accuracy levels that the underlying device supports.
	/// * a `threshold` of zero (0) is a perfect match.
	/// * a `threshold` of 2^32 - 1 is the fuzziest match.
	Fuzzy(MaskedPacketMatcherFields<u32, u32>),
	
	/// A matcher that matches an Internet Control Message Protocol (ICMP) version 4 packet header.
	InternetControlMessageProtocolVersion4Header(MaskedPacketMatcherFields<InternetControlMessageProtocolVersion4HeaderSpecification, InternetControlMessageProtocolVersion4HeaderMask>),
	
	/// A matcher that matches an Internet Control Message Protocol (ICMP) version 6 packet header.
	InternetControlMessageProtocolVersion6Header(MaskedPacketMatcherFields<InternetControlMessageProtocolVersion6HeaderSpecification, InternetControlMessageProtocolVersion6HeaderMask>),
	
	/// A matcher that matches an Internet Protocol (IP) version 4 packet header.
	InternetProtocolVersion4Header(MaskedPacketMatcherFields<InternetProtocolVersion4HeaderSpecification, InternetProtocolVersion4HeaderMask>),
	
	/// A matcher that matches an Internet Protocol (IP) version 6 packet header.
	InternetProtocolVersion6Header(MaskedPacketMatcherFields<InternetProtocolVersion6HeaderSpecification, InternetProtocolVersion6HeaderMask>),
	
	/// A matcher that matches the presence of Internet Protocol (IP) version 6 packet payload's extension header.
	///
	/// Usually preceeded by `InternetProtocolVersion6Header` or itself.
	///
	/// It is not clear what kind of matching this does specifically, and whether it will also match a `NoNextHeader` value or a layer 4 protocol number value.
	InternetProtocolVersion6PayloadExtensionHeaderPresent(MaskedPacketMatcherFields<u8, u8>),
	
	/// Inverts the pattern match, ie acts like a boolean NOT operator.
	Invert,
	
	/// Mark pattern match.
	///
	/// Not all devices will support a mark match, and, of those that do, not all will support the full range from 0 to 2^32 - 1 inclusive.
	///
	/// A mark match matches a packet that has previously been 'marked' with a marking action.
	/// Marks are stored inside the `rte_mbuf` in the same union as the Receive Side Scaling (RSS) hash.
	///
	/// As of DPDK 18.05, this functionality is experimental.
	Mark(MaskedPacketMatcherFields<u32, u32>),
	
	/// Physical Function (PF).
	///
	/// Matches traffic originating from (ingress) or going to (egress) the physical function of the current device.
	PhysicalFunction,
	
	/// Matches traffic originating from (ingress) or going to (egress) a physical port of the underlying device.
	///
	/// The first PhysicalPortPacketMatcher overrides the physical port normally associated with the specified DPDK input port (`port_id`).
	/// This item can be provided several times to match additional physical ports.
	///
	/// Note that physical ports are not necessarily tied to DPDK input ports (`port_id`) when those are not under DPDK control.
	/// Possible values are specific to each device, they are not necessarily indexed from zero and may not be contiguous.
	///
	/// As a device property, the list of allowed values as well as the value associated with a `port_id` should be retrieved by other means.
	PhysicalPort(MaskedPacketMatcherFields<u32, u32>),
	
	/// Matches traffic originating from (ingress) or going to (egress) a given DPDK port identifier (also known as `port_id` and 'port ID').
	///
	/// Normally only supported if the port identifier in question is known by the underlying PMD and related to the device the flow rule is created against.
	///
	/// A port identifier is the application-side way of referring to 'ethernet' connections and getting reference to `eth_dev` structures.
	PortIdentifier(MaskedPacketMatcherFields<u32, u32>),
	
	/// Matches traffic originating from (ingress) or going to (egress) a given virtual function (VF) of the current device.
	///
	/// If supported, should work even if the virtual function (VF) is not managed by the application and thus not associated with a DPDK port identifier (ID).
	///
	/// Note this pattern item does not match VF representors traffic which, as separate entities, should be addressed through their own DPDK port identifiers (IDs).
	///
	/// * Can be specified multiple times to match traffic addressed to several Virtual Function (VF) Identifiers (IDs).
	/// * Can be combined with a PhysicalFunctionPacketMatcher to match both Physical Function (PF) and Virtual Function (VF) traffic.
	VirtualFunction(MaskedPacketMatcherFields<u32, u32>),
	
	/// A matcher that matches either an IEEE 802.1Q Virtual LAN header or an IEEE 802.1ad QinQ Virtual LAN header.
	///
	/// If precedeeded by an EthernetHeaderPacketMatcher, then matches on an IEEE 802.1ad QinQ Virtual LAN header's inner Tag Control Information (TCI).
	VirtualLanHeader(MaskedPacketMatcherFields<VirtualLanHeaderSpecification, VirtualLanHeaderMask>),
	
	/// A 'null' matcher that does nothing.
	Void,
}

impl PacketMatcher
{
	const MaximumPatternMatcher: usize = 16;
	
	/// Flow items.
	///
	/// Resultant array is only valid as long as `packet_matchers` is valid.
	#[inline(always)]
	pub fn rte_flow_items(packet_matchers: &ArrayVec<[PacketMatcher; Self::MaximumPatternMatcher]>) -> ArrayVec<[rte_flow_item; Self::MaximumPatternMatcher]>
	{
		let mut items: ArrayVec<[rte_flow_item; Self::MaximumPatternMatcher]> = ArrayVec::new();
		
		for packet_matcher in packet_matchers
		{
			items.push(packet_matcher.rte_flow_item());
		}
		
		items.push(Self::unspecified_rte_flow_item(rte_flow_item_type::RTE_FLOW_ITEM_TYPE_END));
		
		items
	}
	
	#[inline(always)]
	fn rte_flow_item(&self) -> rte_flow_item
	{
		use self::PacketMatcher::*;
		use self::rte_flow_item_type::*;
		
		match *self
		{
			AddressResolutionProtocolForInternetProtocolVersion4OverEthernet(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			Any(ref masked_packet_matched_fields) => Self::trivially_cast_as_rte_flow_item::<u32, rte_flow_item_any>(RTE_FLOW_ITEM_TYPE_ANY, masked_packet_matched_fields),
			
			EthernetHeader(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			Fuzzy(ref masked_packet_matched_fields) => Self::trivially_cast_as_rte_flow_item::<u32, rte_flow_item_fuzzy>(RTE_FLOW_ITEM_TYPE_FUZZY, masked_packet_matched_fields),
			
			InternetControlMessageProtocolVersion4Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetControlMessageProtocolVersion6Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetProtocolVersion4Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetProtocolVersion6Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetProtocolVersion6PayloadExtensionHeaderPresent(ref masked_packet_matched_fields) => Self::trivially_cast_as_rte_flow_item::<u8, rte_flow_item_ipv6_ext>(RTE_FLOW_ITEM_TYPE_IPV6_EXT, masked_packet_matched_fields),
			
			Invert => Self::unspecified_rte_flow_item(RTE_FLOW_ITEM_TYPE_INVERT),
			
			Mark(ref masked_packet_matched_fields) => Self::trivially_cast_as_rte_flow_item::<u32, rte_flow_item_mark>(RTE_FLOW_ITEM_TYPE_MARK, masked_packet_matched_fields),
			
			PhysicalFunction => Self::unspecified_rte_flow_item(RTE_FLOW_ITEM_TYPE_PF),
			
			PhysicalPort(ref masked_packet_matched_fields) => Self::trivially_cast_as_rte_flow_item::<u32, rte_flow_item_phy_port>(RTE_FLOW_ITEM_TYPE_PHY_PORT, masked_packet_matched_fields),
			
			PortIdentifier(ref masked_packet_matched_fields) => Self::trivially_cast_as_rte_flow_item::<u32, rte_flow_item_port_id>(RTE_FLOW_ITEM_TYPE_PORT_ID, masked_packet_matched_fields),
			
			VirtualFunction(ref masked_packet_matched_fields) => Self::trivially_cast_as_rte_flow_item::<u32, rte_flow_item_vf>(RTE_FLOW_ITEM_TYPE_VF, masked_packet_matched_fields),
			
			VirtualLanHeader(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			Void => Self::unspecified_rte_flow_item(RTE_FLOW_ITEM_TYPE_VOID),
		}
	}
	
	#[inline(always)]
	fn trivially_cast_as_rte_flow_item<S, RteFlowItem>(type_: rte_flow_item_type, masked_packet_matched_fields: &MaskedPacketMatcherFields<S, S>) -> rte_flow_item
	{
		rte_flow_item
		{
			type_,
			spec: &masked_packet_matched_fields.from_specification as *const S as *const RteFlowItem as *const _,
			last: match masked_packet_matched_fields.to_specification
			{
				None => null_mut(),
				Some(ref specification) => specification as *const S as *const RteFlowItem as *const _,
			},
			mask: &masked_packet_matched_fields.mask as *const S as *const RteFlowItem as *const _,
		}
	}
	
	#[inline(always)]
	fn unspecified_rte_flow_item(type_: rte_flow_item_type) -> rte_flow_item
	{
		rte_flow_item
		{
			type_,
			spec: null_mut(),
			last: null_mut(),
			mask: null_mut(),
		}
	}
}


/*
struct rte_flow *
rte_flow_create(uint16_t port_id,
		const struct rte_flow_attr *attr,
		const struct rte_flow_item pattern[],
		const struct rte_flow_action actions[],
		struct rte_flow_error *error);
*/
