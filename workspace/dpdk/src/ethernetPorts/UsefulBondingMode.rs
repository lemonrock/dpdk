// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


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
	AdapativeLoadBalancing,
}

impl UsefulBondingMode
{
	#[inline]
	pub fn modeAndPrimarySlaveAndTransmitPolicy(self) -> (BondingMode, Option<BondingSlave>, Option<BalanceBondingModeTransmitPolicy>)
	{
		match self
		{
			UsefulBondingMode::RoundRobin => (BondingMode::RoundRobin, None, None),
			UsefulBondingMode::ActiveBackup(bondingSlave) => (BondingMode::RoundRobin, Some(bondingSlave), None),
			UsefulBondingMode::Balance(balanceBondingModeTransmitPolicy) => (BondingMode::RoundRobin, None, Some(balanceBondingModeTransmitPolicy)),
			UsefulBondingMode::Broadcast => (BondingMode::Broadcast, None, None),
			UsefulBondingMode::Lacp => (BondingMode::Lacp, None, None),
			UsefulBondingMode::AdaptiveTransmitLoadBalancing => (BondingMode::AdaptiveTransmitLoadBalancing, None, None),
			UsefulBondingMode::AdapativeLoadBalancing => (BondingMode::AdapativeLoadBalancing, None, None),
		}
	}
	
	#[inline]
	pub fn hasPrimarySlave(&self, slaves: &HashSet<BondingSlave>) -> Option<bool>
	{
		match *self
		{
			UsefulBondingMode::ActiveBackup(ref bondingSlave) => Some(slaves.contains(bondingSlave)),
			
			_ => None
		}
	}
}
