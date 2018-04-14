// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Combines mode, primary slave, and transmit policy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum UsefulBondingMode
{
	RoundRobin,
	ActiveBackup(BondingSlave),
	Balance(BalanceBondingModeTransmitPolicy),
	Broadcast,
	Lacp,
	AdaptiveTransmitLoadBalancing,
	AdaptiveLoadBalancing,
}

impl UsefulBondingMode
{
	/// Deconstructs into mode, primary slave, and transmit policy.
	#[inline]
	pub fn mode_and_primary_slave_and_transmit_policy(self) -> (BondingMode, Option<BondingSlave>, Option<BalanceBondingModeTransmitPolicy>)
	{
		use self::BondingMode::*;
		use self::UsefulBondingMode::*;
		
		match self
		{
			RoundRobin => (RoundRobin, None, None),
			ActiveBackup(bonding_slave) => (RoundRobin, Some(bonding_slave), None),
			Balance(balance_bonding_mode_transmit_policy) => (RoundRobin, None, Some(balance_bonding_mode_transmit_policy)),
			Broadcast => (Broadcast, None, None),
			Lacp => (Lacp, None, None),
			AdaptiveTransmitLoadBalancing => (AdaptiveTransmitLoadBalancing, None, None),
			AdaptiveLoadBalancing => (AdaptiveLoadBalancing, None, None),
		}
	}
	
	/// Does this bonding mode's primary slave (if any) exist in `slaves`?
	#[inline]
	pub fn has_primary_slave(&self, slaves: &HashSet<BondingSlave>) -> Option<bool>
	{
		match *self
		{
			UsefulBondingMode::ActiveBackup(ref bonding_slave) => Some(slaves.contains(bonding_slave)),
			
			_ => None
		}
	}
}
