// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub enum LinkSpeeds
{
	AutonegotiationOfAllSupportedSpeeds,
	
	AutonegotiationOfTheseSpeeds(HashSet<LinkSpeed>),
	
	FixedWithoutAutonegotiation(LinkSpeed),
}

impl LinkSpeeds
{
	#[inline(always)]
	pub fn as_uint32_t(&self) -> uint32_t
	{
		match *self
		{
			LinkSpeeds::AutonegotiationOfAllSupportedSpeeds => ::dpdk_sys::ETH_LINK_SPEED_AUTONEG,
			LinkSpeeds::AutonegotiationOfTheseSpeeds(ref linkSpeeds) =>
			{
				let mut bitsSet = ::dpdk_sys::ETH_LINK_SPEED_AUTONEG;
				for linkSpeed in linkSpeeds
				{
					bitsSet |= *linkSpeed as u32;
				}
				bitsSet
			},
			LinkSpeeds::FixedWithoutAutonegotiation(ref fixedLinkSpeed) => *fixedLinkSpeed as u32 | ::dpdk_sys::ETH_LINK_SPEED_FIXED,
		}
	}
}

impl Default for LinkSpeeds
{
	#[inline(always)]
	fn default() -> Self
	{
		LinkSpeeds::AutonegotiationOfAllSupportedSpeeds
	}
}
