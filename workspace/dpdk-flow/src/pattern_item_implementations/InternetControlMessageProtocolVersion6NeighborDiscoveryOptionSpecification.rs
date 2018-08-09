// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Control Message Protocol (ICMP) version 4 Neighbor Discovery option specification which shadows InternetControlMessageProtocolVersion6NeighborDiscoveryOption.
pub type InternetControlMessageProtocolVersion6NeighborDiscoveryOptionSpecification = InternetControlMessageProtocolVersion6NeighborDiscoveryOption;

impl MaskedPattern for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionSpecification
{
	type Type = rte_flow_item_icmp6_nd_opt;
}

impl Specification for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionSpecification
{
	const DpdkFlowType: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_ICMP6_ND_OPT;
	
	type Mask = InternetControlMessageProtocolVersion6NeighborDiscoveryOptionMask;
	
	#[inline(always)]
	fn dpdk_specification(&self) -> &<Self as MaskedPattern>::Type
	{
		unsafe { transmute(self) }
	}
}
