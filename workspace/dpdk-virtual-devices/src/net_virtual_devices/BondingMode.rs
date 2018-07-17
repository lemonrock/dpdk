// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a bonding mode.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BondingMode
{
	/// Round-Robin.
	RoundRobin = 0,
	
	/// Active-Backup.
	ActiveBackup = 1,
	
	/// Balance.
	Balance = 2,
	
	/// Broadcast.
	Broadcast = 3,
	
	/// IEEE 802.23ad Link Aggregation Control Protocol (LACP).
	Lacp = 4,
	
	/// Adaptive transmit load-balancing.
	AdaptiveTransmitLoadBalancing = 5,
	
	/// Load-balancing.
	AdaptiveLoadBalancing = 6,
}
