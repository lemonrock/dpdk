// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Matches any protocol in place of the current layer, a single instance may also stand for several protocol layers.
///
/// This is usually specified as the first pattern item when looking for a protocol anywhere in a packet.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndPacketMatcher;

impl PacketMatcher for EndPacketMatcher
{
	type DpdkType = ();
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_END;
	
	const IsMeta: bool = true;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		&()
	}
}
