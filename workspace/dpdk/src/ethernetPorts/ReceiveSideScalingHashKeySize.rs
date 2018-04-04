// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Based on the enum rte_filter_type but without the noise and invalid values
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveSideScalingHashKeySize
{
	Forty = 40,
	FiftyTwo = 52,
}

impl ReceiveSideScalingHashKeySize
{
	#[inline(always)]
	pub fn fromNumber(supportedHashKeySize: u8) -> Option<Self>
	{
		match supportedHashKeySize
		{
			40 => Some(ReceiveSideScalingHashKeySize::Forty),
			52 => Some(ReceiveSideScalingHashKeySize::FiftyTwo),
			
			_ => None,
		}
	}
	
	#[inline(always)]
	pub fn fromNumberOrPanic(supportedHashKeySize: u8) -> Self
	{
		Self::fromNumber(supportedHashKeySize).expect("Only 40 and 52 byte long keys are currently supported")
	}
	
	#[inline(always)]
	pub fn fromNumberOrPanicAndZeroLengthIsNone(supportedHashKeySize: u8) -> Option<Self>
	{
		if supportedHashKeySize == 0
		{
			None
		}
		else
		{
			Some(Self::fromNumberOrPanic(supportedHashKeySize))
		}
	}
}
