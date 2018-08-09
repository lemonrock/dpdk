// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask for an `PacketMatcher::InternetControlMessageProtocolVersion6NeighborDiscoveryOption`.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6NeighborDiscoveryOptionMask
{
	/// Type mask (see [IANA](https://www.iana.org/assignments/icmpv6-parameters/icmpv6-parameters.xhtml#icmpv6-parameters-5>)).
	pub type_: u8,
	
	/// Length mask.
	pub target_address: u8,
}

impl MaskedPacketMatcher for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionMask
{
	type Type = rte_flow_item_icmp6_nd_opt;
}

impl Mask for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		unsafe { transmute(self) }
	}
}
