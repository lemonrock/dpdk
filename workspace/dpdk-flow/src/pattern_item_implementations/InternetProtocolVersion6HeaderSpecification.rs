// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Protocol (IP) version 6 header specification which shadows InternetProtocolVersion4PacketHeader.
pub type InternetProtocolVersion6HeaderSpecification = InternetProtocolVersion6PacketHeader;

impl MaskedPacketMatcher for InternetProtocolVersion6HeaderSpecification
{
	type Type = rte_flow_item_ipv6;
}

impl Specification for InternetProtocolVersion6HeaderSpecification
{
	const DpdkFlowType: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_IPV6;
	
	type Mask = InternetProtocolVersion6HeaderMask;
	
	#[inline(always)]
	fn dpdk_specification(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		unsafe { transmute(self) }
	}
}
