// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrafficClass
{
	pub differentiatedServiceCodePoint: DifferentiatedServiceCodePoint,
	pub explicitCongestionNotification: ExplicitCongestionNotification,
}

impl TrafficClass
{
	#[inline(always)]
	pub fn as_u8(&self) -> u8
	{
		self.differentiatedServiceCodePoint.0 << 2 | self.explicitCongestionNotification as u8
	}
}
