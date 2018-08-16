// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Depth (in tunnel layers) into a packet to go for its Receive Side Scaling (RSS) hash calculation.
///
/// Defaults to `NetworkCardChoiceButUsuallyInnermost`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum ReceiveSideScalingLevel
{
	/// Calculates Receive Side Scaling (RSS) hash using the 'innermost' part of a packet; only relevant when a packet is tunnelled.
	NetworkCardChoiceButUsuallyInnermost,
	
	/// Calculates Receive Side Scaling (RSS) hash using the 'outermost' part of a packet.
	///
	/// Not necessarily supported.
	Outermost,
	
	/// Specifies how many inner layers of a tunnelled packet to ignore before calculating a Receive Side Scaling (RSS) hash.
	///
	/// The outermost layer is always ignored.
	///
	/// Rarely supported.
	///
	/// (Internally, we add 2 to this value).
	IgnoreOutermostAndThisManyInnerLayers(u32),
}

impl Into<u32> for ReceiveSideScalingLevel
{
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::ReceiveSideScalingLevel::*;
		
		match self
		{
			NetworkCardChoiceButUsuallyInnermost => 0,
			Outermost => 1,
			IgnoreOutermostAndThisManyInnerLayers(value) => 2 + value,
		}
	}
}

impl From<u32> for ReceiveSideScalingLevel
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		use self::ReceiveSideScalingLevel::*;
		
		match value
		{
			0 => NetworkCardChoiceButUsuallyInnermost,
			1 => Outermost,
			_ => IgnoreOutermostAndThisManyInnerLayers(value - 2)
		}
	}
}

impl Default for ReceiveSideScalingLevel
{
	#[inline(always)]
	fn default() -> Self
	{
		ReceiveSideScalingLevel::NetworkCardChoiceButUsuallyInnermost
	}
}
