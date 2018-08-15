// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Balance bonding mode transmit policy for bonded network devices.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum BalanceBondingModeTransmitPolicy
{
	/// Layer 2 only.
	Layer2Only = 0,
	
	/// Layers 2 and 3.
	Layers2And3 = 1,
	
	/// Layers 3 and 4.
	Layers3And4 = 2,
}

impl Default for BalanceBondingModeTransmitPolicy
{
	#[inline(always)]
	fn default() -> Self
	{
		BalanceBondingModeTransmitPolicy::Layers3And4
	}
}
