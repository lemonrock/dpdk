// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive Side Scaling (RSS) hash key size in bytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveSideScalingHashKeySize
{
	/// 40 bytes.
	Forty,
	
	/// 52 bytes.
	FiftyTwo,
}

impl Default for ReceiveSideScalingHashKeySize
{
	#[inline(always)]
	fn default() -> Self
	{
		ReceiveSideScalingHashKeySize::Forty
	}
}
