// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinkSpeed
{
	_10MbpsHalfDuplex = ETH_LINK_SPEED_10M_HD,
	_10Mbps = ETH_LINK_SPEED_10M,
	_100MbpsHalfDuplex = ETH_LINK_SPEED_100M_HD,
	_100Mbps = ETH_LINK_SPEED_100M,
	_1Gbps = ETH_LINK_SPEED_1G,
	_2AndAHalfGbps = ETH_LINK_SPEED_2_5G,
	_5Gbps = ETH_LINK_SPEED_5G,
	_10Gbps = ETH_LINK_SPEED_10G,
	_20Gbps = ETH_LINK_SPEED_20G,
	_25Gbps = ETH_LINK_SPEED_25G,
	_40Gbps = ETH_LINK_SPEED_40G,
	_50Gbps = ETH_LINK_SPEED_50G,
	_56Gbps = ETH_LINK_SPEED_56G,
	_100Gbps = ETH_LINK_SPEED_100G,
}

impl LinkSpeed
{
	#[inline(always)]
	pub fn isHalfDuplex(&self) -> bool
	{
		match *self
		{
			LinkSpeed::_10MbpsHalfDuplex => true,
			LinkSpeed::_100MbpsHalfDuplex => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub fn isFullDuplex(&self) -> bool
	{
		match *self
		{
			LinkSpeed::_10MbpsHalfDuplex => false,
			LinkSpeed::_100MbpsHalfDuplex => false,
			_ => true,
		}
	}
	
	#[inline(always)]
	pub fn fromLinkStatusPanicOnInvalidValue(linkSpeedInMbps: u32, isFullDuplex: bool, isUp: bool) -> Option<LinkSpeed>
	{
		match linkSpeedInMbps
		{
			ETH_SPEED_NUM_NONE =>
			{
				assert!(!isUp, "isUp is true for a linkSpeedInMbps '0'");
				assert!(isFullDuplex, "isFullDuplex is false (ie half duplex) for a linkSpeedInMbps '0'");
				
				None
			},
			
			ETH_SPEED_NUM_10M =>
			{
				assert!(isUp, "isUp is false for a linkSpeedInMbps '{}'", linkSpeedInMbps);
				
				Some
				(
					if isFullDuplex
					{
						LinkSpeed::_10Mbps
					}
					else
					{
						LinkSpeed::_10MbpsHalfDuplex
					}
				)
			},
			
			ETH_SPEED_NUM_100M =>
			{
				assert!(isUp, "isUp is false for a linkSpeedInMbps '{}'", linkSpeedInMbps);
				
				Some
				(
					if isFullDuplex
					{
						LinkSpeed::_100Mbps
					}
					else
					{
						LinkSpeed::_100MbpsHalfDuplex
					}
				)
			}
			
			_ =>
			{
				assert!(isUp, "isUp is false for a linkSpeedInMbps '{}'", linkSpeedInMbps);
				assert!(isFullDuplex, "isFullDuplex is false (ie half duplex) for a linkSpeedInMbps of '{}'", linkSpeedInMbps);
				
				Some
				(
					match linkSpeedInMbps
					{
						ETH_SPEED_NUM_1G => LinkSpeed::_1Gbps,
						ETH_SPEED_NUM_2_5G => LinkSpeed::_2AndAHalfGbps,
						ETH_SPEED_NUM_5G => LinkSpeed::_5Gbps,
						ETH_SPEED_NUM_10G => LinkSpeed::_10Gbps,
						ETH_SPEED_NUM_20G => LinkSpeed::_20Gbps,
						ETH_SPEED_NUM_25G => LinkSpeed::_25Gbps,
						ETH_SPEED_NUM_40G => LinkSpeed::_40Gbps,
						ETH_SPEED_NUM_50G => LinkSpeed::_50Gbps,
						ETH_SPEED_NUM_56G => LinkSpeed::_56Gbps,
						ETH_SPEED_NUM_100G => LinkSpeed::_100Gbps,
			
						_ => panic!("The linkSpeedInMbps '{}' is not a valid value", linkSpeedInMbps),
					}
				)
			}
		}
	}
}
