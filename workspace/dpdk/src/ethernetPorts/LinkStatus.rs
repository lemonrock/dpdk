// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkStatus
{
	speed: LinkSpeed,
	isAutoNegotiated: bool,
}

impl LinkStatus
{
	#[inline(always)]
	pub fn from_rte_eth_link(linkDetails: &rte_eth_link) -> Option<Self>
	{
		let isUp = match unsafe { rust_rte_eth_link_getBitField_link_status(linkDetails) }
		{
			ETH_LINK_DOWN => false,
			ETH_LINK_UP => true,
			illegal @ _ => panic!("Invalid link_status bitfield '{}'", illegal),
		};

		if !isUp
		{
			return None;
		}

		let isFullDuplex = match unsafe { rust_rte_eth_link_getBitField_link_duplex(linkDetails) }
		{
			ETH_LINK_HALF_DUPLEX => false,
			ETH_LINK_FULL_DUPLEX => true,
			illegal @ _ => panic!("Invalid link_duplex bitfield '{}'", illegal),
		};

		let isAutoNegotiated = match unsafe { rust_rte_eth_link_getBitField_link_autoneg(linkDetails) }
		{
			ETH_LINK_FIXED => false,
			ETH_LINK_AUTONEG => true,
			illegal @ _ => panic!("Invalid link_autoneg bitfield '{}'", illegal),
		};

		Some
		(
			LinkStatus
			{
				speed: LinkSpeed::fromLinkStatusPanicOnInvalidValue(linkDetails.link_speed, isFullDuplex, isUp).unwrap(),
				isAutoNegotiated,
			}
		)
	}

	#[inline(always)]
	pub fn isAutoNegotiated(&self) -> bool
	{
		self.isAutoNegotiated
	}

	#[inline(always)]
	pub fn isFixed(&self) -> bool
	{
		!self.isAutoNegotiated
	}
}
