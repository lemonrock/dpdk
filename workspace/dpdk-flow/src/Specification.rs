// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification.
pub trait Specification: MaskedPacketMatcher
{
	#[doc(hidden)]
	const DpdkFlowType: rte_flow_item_type;
	
	#[doc(hidden)]
	type Mask: Mask<Type=<Self as MaskedPacketMatcher>::Type>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn dpdk_specification(&self) -> &<Self as MaskedPacketMatcher>::Type;
}
