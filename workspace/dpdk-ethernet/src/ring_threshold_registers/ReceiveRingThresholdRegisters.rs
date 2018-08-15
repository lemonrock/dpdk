// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ring threshold registers for receive queues.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct ReceiveRingThresholdRegisters
{
	/// Prefetch threshold.
	pub prefetch_threshold: NonZeroU8,
	
	/// Host threshold.
	pub host_threshold: NonZeroU8,
	
	/// Write-back threshold.
	pub write_back_threshold: NonZeroU8,
}

impl From<rte_eth_thresh> for ReceiveRingThresholdRegisters
{
	#[inline(always)]
	fn from(value: rte_eth_thresh) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl Into<rte_eth_thresh> for ReceiveRingThresholdRegisters
{
	#[inline(always)]
	fn into(self) -> rte_eth_thresh
	{
		unsafe { transmute(self) }
	}
}

impl RingThresholdRegisters for ReceiveRingThresholdRegisters
{
}
