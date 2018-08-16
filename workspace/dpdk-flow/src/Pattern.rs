// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Patterns which match properties of packets, or, in some cases, control subsequent patterns.
///
/// The following patterns affect subsequent patterns:-
///
/// * `Invert`
/// * `Fuzzy`
///
/// The following patterns should be specified at the start of the list:-
///
/// * `Invert`
/// * `Fuzzy`
/// * `Any`
/// * `PhysicalFunction`
/// * `VirtualFunction`
/// * `PhysicalPort`
/// * `PortIdentifier`
///
/// The following DPDK patterns are not yet implemented but are planned to be supported:-
///
/// * `ICMP6_ND_OPT_SLA_ETH`
/// * `ICMP6_ND_OPT_TLA_ETH`
/// * `TCP`
/// * `UDP`
///
/// The following DPDK patterns are not yet implemented and there are no plans to support them:-
///
/// * `E_TAG`
/// * `ESP`
/// * `GENEVE`
/// * `GRE`
/// * `GTP`
/// * `GTPC`
/// * `GTPU`
/// * `MPLS`
/// * `NVGRE`
/// * `SCTP`
/// * `VXLAN`
/// * `VXLAN_GPE`
///
/// Note that the `END` DPDK pattern is applied automatically.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum Pattern
{
	/// A matcher that matches an Address Resolution Protocol (ARP) Internet Protocol (IP) version 4 packet over Ethernet.
	///
	/// The underlying DPDK functionality supports other kinds of ARP headers but always assumes an InternetProtocolVersion4-sized payload!
	AddressResolutionProtocolForInternetProtocolVersion4OverEthernet(MaskedPatternFields<AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification, AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask>),
	
	/// Matches at a number of layers.
	Any(MaskedPatternFields<u32, u32>),
	
	/// A matcher that matches an ethernet header.
	///
	/// When followed by a 'layer 2.5' matcher such as VirtualLanHeaderPattern, the Ether Type is a tag protocol identifier (TPID).
	/// In this case, the ether type refers to the outer header, with the VirtualLanHeaderPattern's ether type referring to the inner Ether Type or tag protocol identifier (TPID).
	EthernetHeader(MaskedPatternFields<EthernetHeaderSpecification, EthernetHeaderMask>),
	
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
	Fuzzy(MaskedPatternFields<u32, u32>),
	
	/// A matcher that matches an Internet Control Message Protocol (ICMP) version 4 packet header.
	InternetControlMessageProtocolVersion4Header(MaskedPatternFields<InternetControlMessageProtocolVersion4HeaderSpecification, InternetControlMessageProtocolVersion4HeaderMask>),
	
	/// A matcher that matches an Internet Control Message Protocol (ICMP) version 6 packet header.
	InternetControlMessageProtocolVersion6Header(MaskedPatternFields<InternetControlMessageProtocolVersion6HeaderSpecification, InternetControlMessageProtocolVersion6HeaderMask>),
	
	/// A matcher that matches an Internet Control Message Protocol (ICMP) version 6 Neigbor Discovery Advertisement packet.
	InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisement(MaskedPatternFields<InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementSpecification, InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementMask>),
	
	/// A matcher that matches an Internet Control Message Protocol (ICMP) version 6 Neigbor Discovery Solicitation packet.
	InternetControlMessageProtocolVersion6NeighborDiscoverySolicitation(MaskedPatternFields<InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationSpecification, InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask>),
	
	/// A matcher that matches an Internet Control Message Protocol (ICMP) version 6 Neigbor Discovery option.
	InternetControlMessageProtocolVersion6NeighborDiscoveryOption(MaskedPatternFields<InternetControlMessageProtocolVersion6NeighborDiscoveryOptionSpecification, InternetControlMessageProtocolVersion6NeighborDiscoveryOptionMask>),
	
	/// A matcher that matches an Internet Protocol (IP) version 4 packet header.
	InternetProtocolVersion4Header(MaskedPatternFields<InternetProtocolVersion4HeaderSpecification, InternetProtocolVersion4HeaderMask>),
	
	/// A matcher that matches an Internet Protocol (IP) version 6 packet header.
	InternetProtocolVersion6Header(MaskedPatternFields<InternetProtocolVersion6HeaderSpecification, InternetProtocolVersion6HeaderMask>),
	
	/// A matcher that matches the presence of Internet Protocol (IP) version 6 packet payload's extension header.
	///
	/// Usually preceeded by `InternetProtocolVersion6Header` or itself.
	///
	/// It is not clear what kind of matching this does specifically, and whether it will also match a `NoNextHeader` value or a layer 4 protocol number value.
	InternetProtocolVersion6PayloadExtensionHeaderPresent(MaskedPatternFields<u8, u8>),
	
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
	Mark(MaskedPatternFields<u32, u32>),
	
	/// Physical Function (PF).
	///
	/// Matches traffic originating from (ingress) or going to (egress) the physical function of the current device.
	PhysicalFunction,
	
	/// Matches traffic originating from (ingress) or going to (egress) a physical port of the underlying device (eg on a 4-port card, one of ports 1 to 4 but see caveats below).
	///
	/// This item can be provided several times to match additional physical ports.
	///
	/// Note that physical ports are not necessarily tied to DPDK input port identifierss (`port_id`) when those are not under DPDK control.
	/// Possible values are specific to each device, they are not necessarily indexed from zero and may not be contiguous.
	///
	/// As a device property, the list of allowed values as well as the value associated with a `port_id` should be retrieved by other means.
	PhysicalPort(MaskedPatternFields<u32, u32>),
	
	/// Matches traffic originating from (ingress) or going to (egress) a given DPDK port identifier (also known as `port_id` and 'port ID') or `EthernetPortIdentifier`.
	///
	/// Normally only supported if the port identifier in question is known by the underlying PMD and related to the device the flow rule is created against.
	///
	/// A port identifier is the application-side way of referring to 'ethernet' connections and getting reference to `eth_dev` structures.
	PortIdentifier(MaskedPatternFields<EthernetPortIdentifier, u16>),
	
	/// Matches a byte string of a given length at a given offset.
	///
	/// Offset is either absolute (using the start of the packet) or relative to the end of the previous matched item in the stack, in which case negative values are allowed.
	///
	/// If search is enabled, offset is used as the starting point.
	/// The search area can be delimited by setting `search_area_limit_for_start_of_pattern` to a nonzero value, which is the maximum number of bytes after offset where the pattern may start.
	///
	/// Matching a zero-length pattern is allowed, doing so resets the relative offset for subsequent items.
	///
	/// This type does not support ranges.
	Raw(RawSpecification, RawMask),
	
	/// Matches traffic originating from (ingress) or going to (egress) a given virtual function (VF) of the current device.
	///
	/// If supported, should work even if the virtual function (VF) is not managed by the application and thus not associated with a DPDK port identifier (ID).
	///
	/// Note this pattern item does not match VF representors traffic which, as separate entities, should be addressed through their own DPDK port identifiers (IDs).
	///
	/// * Can be specified multiple times to match traffic addressed to several Virtual Function (VF) Identifiers (IDs).
	/// * Can be combined with a PhysicalFunctionPattern to match both Physical Function (PF) and Virtual Function (VF) traffic.
	VirtualFunction(MaskedPatternFields<u32, u32>),
	
	/// A matcher that matches either an IEEE 802.1Q Virtual LAN header or an IEEE 802.1ad QinQ Virtual LAN header.
	///
	/// If precedeeded by an EthernetHeaderPattern, then matches on an IEEE 802.1ad QinQ Virtual LAN header's inner Tag Control Information (TCI).
	VirtualLanHeader(MaskedPatternFields<VirtualLanHeaderSpecification, VirtualLanHeaderMask>),
	
	/// A 'null' matcher that does nothing.
	Void,
}

impl Pattern
{
	const MaximumPatterns: usize = 16;
	
	#[inline(always)]
	pub(crate) fn rte_flow_items(packet_matchers: &ArrayVec<[Box<Pattern>; Self::MaximumPatterns]>, drop_prevention: &mut Vec<Box<Any>>) -> ArrayVec<[rte_flow_item; Self::MaximumPatterns]>
	{
		let mut items: ArrayVec<[rte_flow_item; Self::MaximumPatterns]> = ArrayVec::new();
		
		for packet_matcher in packet_matchers
		{
			items.push(packet_matcher.rte_flow_item(drop_prevention));
		}
		
		items.push(Self::unspecified_rte_flow_item(rte_flow_item_type::RTE_FLOW_ITEM_TYPE_END));
		
		items
	}
	
	#[inline(always)]
	fn rte_flow_item(&self, drop_prevention: &mut Vec<Box<Any>>) -> rte_flow_item
	{
		use self::Pattern::*;
		use self::rte_flow_item_type::*;
		
		match *self
		{
			AddressResolutionProtocolForInternetProtocolVersion4OverEthernet(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			Any(ref masked_packet_matched_fields) => masked_packet_matched_fields.trivially_cast_as_rte_flow_item::<rte_flow_item_any>(RTE_FLOW_ITEM_TYPE_ANY),
			
			EthernetHeader(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			Fuzzy(ref masked_packet_matched_fields) => masked_packet_matched_fields.trivially_cast_as_rte_flow_item::<rte_flow_item_fuzzy>(RTE_FLOW_ITEM_TYPE_FUZZY),
			
			InternetControlMessageProtocolVersion4Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetControlMessageProtocolVersion6Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisement(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetControlMessageProtocolVersion6NeighborDiscoverySolicitation(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetControlMessageProtocolVersion6NeighborDiscoveryOption(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetProtocolVersion4Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetProtocolVersion6Header(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			InternetProtocolVersion6PayloadExtensionHeaderPresent(ref masked_packet_matched_fields) => masked_packet_matched_fields.trivially_cast_as_rte_flow_item::<rte_flow_item_ipv6_ext>(RTE_FLOW_ITEM_TYPE_IPV6_EXT),
			
			Invert => Self::unspecified_rte_flow_item(RTE_FLOW_ITEM_TYPE_INVERT),
			
			Mark(ref masked_packet_matched_fields) => masked_packet_matched_fields.trivially_cast_as_rte_flow_item::<rte_flow_item_mark>(RTE_FLOW_ITEM_TYPE_MARK),
			
			PhysicalFunction => Self::unspecified_rte_flow_item(RTE_FLOW_ITEM_TYPE_PF),
			
			PhysicalPort(ref masked_packet_matched_fields) => masked_packet_matched_fields.trivially_cast_as_rte_flow_item::<rte_flow_item_phy_port>(RTE_FLOW_ITEM_TYPE_PHY_PORT),
			
			PortIdentifier(ref masked_packet_matched_fields) =>
			{
				#[inline(always)]
				fn from_ethernet_port_identifier(ethernet_port_identifier: EthernetPortIdentifier, drop_prevention: &mut Vec<Box<::std::any::Any>>) -> *const c_void
				{
					to_u32_pointer(ethernet_port_identifier.into(), drop_prevention)
				}
				
				#[inline(always)]
				fn to_u32_pointer(value: Box<u32>, drop_prevention: &mut Vec<Box<::std::any::Any>>) -> *const c_void
				{
					let pointer = value.as_ref() as *const u32 as *const c_void;
					drop_prevention.push(value);
					pointer
				}
				
				rte_flow_item
				{
					type_: RTE_FLOW_ITEM_TYPE_PORT_ID,
					spec: from_ethernet_port_identifier(masked_packet_matched_fields.from_specification, drop_prevention),
					last: match masked_packet_matched_fields.to_specification
					{
						None => null_mut(),
						Some(specification) => from_ethernet_port_identifier(specification, drop_prevention),
					},
					mask: to_u32_pointer(Box::new(masked_packet_matched_fields.mask as u32), drop_prevention),
				}
			}
			
			Raw(ref specification, ref mask) =>
			{
				rte_flow_item
				{
					type_: RawSpecification::DpdkFlowType,
					spec: specification.dpdk_specification() as *const <RawSpecification as MaskedPattern>::Type as *const _,
					last: null_mut(),
					mask: mask.dpdk_mask() as *const <RawSpecification as MaskedPattern>::Type as *const _,
				}
			}
			
			VirtualFunction(ref masked_packet_matched_fields) => masked_packet_matched_fields.trivially_cast_as_rte_flow_item::<rte_flow_item_vf>(RTE_FLOW_ITEM_TYPE_VF),
			
			VirtualLanHeader(ref masked_packet_matched_fields) => masked_packet_matched_fields.rte_flow_item(),
			
			Void => Self::unspecified_rte_flow_item(RTE_FLOW_ITEM_TYPE_VOID),
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
