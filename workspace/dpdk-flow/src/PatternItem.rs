// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A masked packet matcher.
pub trait MaskedPacketMatcher
{
	#[doc(hidden)]
	type Type;
}

/// Specification.
pub trait Specification: MaskedPacketMatcher
{
	#[doc(hidden)]
	const DpdkFlowType: rte_flow_item_type;
	
	#[doc(hidden)]
	type Mask: Mask<Type=<Self as MaskedPacketMatcher>::Type>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn dpdk_specification(&self) -> NonNull<<Self as MaskedPacketMatcher>::Type>;
}

/// Mask.
pub trait Mask: MaskedPacketMatcher
{
	#[doc(hidden)]
	#[inline(always)]
	fn dpdk_mask(&self) -> NonNull<<Self as MaskedPacketMatcher>::Type>;
}

/// Commonly reocurring fields for a masked packet matcher
pub struct MaskedPacketMatcherFields<S: Specification>
{
	from_specification: S,
	to_specification: Option<S>,
	mask: S::Mask,
}

impl<S: Specification> MaskedPacketMatcherFields<S>
{
	#[inline(always)]
	fn rte_flow_item(&self) -> rte_flow_item
	{
		rte_flow_item
		{
			type_: S::DpdkFlowType,
			spec: self.from_specification.dpdk_specification().as_ptr() as *const S::Type as *const _,
			last: match self.to_specification
			{
				None => null_mut(),
				Some(ref specification) => specification.dpdk_specification().as_ptr() as *const S::Type as *const _,
			},
			mask: self.mask.dpdk_mask().as_ptr() as *const S::Type as *const _,
		}
	}
}

/// Packet matchers.
pub enum PacketMatcher
{
	/// A matcher that matches an Address Resolution Protocol (ARP) Internet Protocol (IP) version 4 packet over Ethernet.
	///
	/// The underlying DPDK functionality supports other kinds of ARP headers but always assumes an InternetProtocolVersion4-sized payload!
	AddressResolutionProtocolForInternetProtocolVersion4OverEthernet(MaskedPacketMatcherFields<AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification>),
	
//	#[allow(doc_missing)]
//	Any(MaskedPacketMatcherFields<rte_flow_item_any>),
	
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


/*
struct rte_flow *
rte_flow_create(uint16_t port_id,
		const struct rte_flow_attr *attr,
		const struct rte_flow_item pattern[],
		const struct rte_flow_action actions[],
		struct rte_flow_error *error);
*/
