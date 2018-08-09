// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Commonly reocurring fields for a masked packet matcher
pub struct MaskedPacketMatcherFields<S, M>
{
	from_specification: S,
	to_specification: Option<S>,
	mask: M,
}

impl<S> MaskedPacketMatcherFields<S, S>
{
	#[inline(always)]
	pub(crate) fn trivially_cast_as_rte_flow_item<RteFlowItem>(&self, type_: rte_flow_item_type) -> rte_flow_item
	{
		rte_flow_item
		{
			type_,
			spec: &self.from_specification as *const S as *const RteFlowItem as *const _,
			last: match self.to_specification
			{
				None => null_mut(),
				Some(ref specification) => specification as *const S as *const RteFlowItem as *const _,
			},
			mask: &self.mask as *const S as *const RteFlowItem as *const _,
		}
	}
}

impl<S: Specification> MaskedPacketMatcherFields<S, S::Mask>
{
	#[inline(always)]
	pub(crate) fn rte_flow_item(&self) -> rte_flow_item
	{
		rte_flow_item
		{
			type_: S::DpdkFlowType,
			spec: self.from_specification.dpdk_specification() as *const S::Type as *const _,
			last: match self.to_specification
			{
				None => null_mut(),
				Some(ref specification) => specification.dpdk_specification() as *const S::Type as *const _,
			},
			mask: self.mask.dpdk_mask() as *const S::Type as *const _,
		}
	}
	
}
