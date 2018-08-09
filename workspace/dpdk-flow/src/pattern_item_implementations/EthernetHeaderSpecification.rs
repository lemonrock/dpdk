// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet header specification which shadows EthernetPacketHeader.
pub type EthernetHeaderSpecification = EthernetPacketHeader;

impl MaskedPacketMatcher for EthernetHeaderSpecification
{
	type Type = rte_flow_item_eth;
}

impl Specification for EthernetHeaderSpecification
{
	const DpdkFlowType: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_ETH;
	
	type Mask = EthernetHeaderMask;
	
	#[inline(always)]
	fn dpdk_specification(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		unsafe { transmute(self) }
	}
}
