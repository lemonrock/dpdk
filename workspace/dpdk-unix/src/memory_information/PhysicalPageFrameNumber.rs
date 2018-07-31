// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Physical Page Frame Number (PFN).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PhysicalPageFrameNumber(pub(crate) u64);

impl Into<u64> for PhysicalPageFrameNumber
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Into<PhysicalAddress> for PhysicalPageFrameNumber
{
	#[inline(always)]
	fn into(self) -> PhysicalAddress
	{
		PhysicalAddress(self.0 * (page_size() as u64))
	}
}
