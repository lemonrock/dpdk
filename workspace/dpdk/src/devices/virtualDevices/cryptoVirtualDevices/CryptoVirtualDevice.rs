// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait CryptoVirtualDevice : VirtualDevice
{
	const DefaultMaximumNumberOfQueuePairs: u31 = 8;
	const DefaultMaximumNumberOfSessions: u31 = 2048;
	
	#[inline(always)]
	fn maximNumberOfQueuePairs(&self) -> u31;
	
	#[inline(always)]
	fn maximNumberOfSessions(&self) -> u31;
	
	#[inline(always)]
	fn numaSocketId(&self) -> NumaSocketId;
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingCommaCommon(&self) -> String
	{
		format!(",max_nb_queue_pairs={},max_nb_sessions={},socket_id={}", self.maximNumberOfQueuePairs(), self.maximNumberOfSessions(), self.numaSocketId().as_u8())
	}
}
