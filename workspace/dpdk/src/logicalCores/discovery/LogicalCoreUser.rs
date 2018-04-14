// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalCoreUser
{
	preferredNumaSocketId: Option<NumaSocketId>,
	wouldLikeToMakeUseOf: usize,
	canMakeUseOf: usize,
	canStillMakeUseOf: usize,
	uses: Vec<LogicalCore>,
}

impl LogicalCoreUser
{
	pub fn newForNonEthernetThreads(wouldLikeToMakeUseOf: usize) -> Self
	{
		Self::new(None, wouldLikeToMakeUseOf)
	}

	pub fn new(numa_socket_id: Option<NumaSocketId>, wouldLikeToMakeUseOf: usize) -> Self
	{
		assert_ne!(wouldLikeToMakeUseOf, 0, "wouldLikeToMakeUseOf can not be zero");

		let canMakeUseOf = min(wouldLikeToMakeUseOf, MaximumLogicalCores);

		LogicalCoreUser
		{
			preferredNumaSocketId: numa_socket_id,
			wouldLikeToMakeUseOf,
			canMakeUseOf,
			canStillMakeUseOf: canMakeUseOf,
			uses: Vec::with_capacity(canMakeUseOf)
		}
	}

	pub fn willMakeUseOf(&mut self, numa_socket_id: NumaSocketId, logicalCore: LogicalCore) -> bool
	{
		if self.canStillMakeUseOf == 0
		{
			return false;
		}

		if self.preferredNumaSocketId.is_none() || self.preferredNumaSocketId.unwrap() == numa_socket_id
		{
			self.uses.push(logicalCore);
			self.canStillMakeUseOf -= 1;
			if self.canStillMakeUseOf == 0
			{
				self.uses.shrink_to_fit();
				true
			}
			else
			{
				false
			}
		}
		else
		{
			false
		}
	}

	pub fn willMakeUseOfForNonLocalNumaNode(&mut self, logicalCore: LogicalCore) -> bool
	{
		if self.canStillMakeUseOf == 0
		{
			return false;
		}

		self.uses.push(logicalCore);
		self.canStillMakeUseOf -= 1;
		if self.canStillMakeUseOf == 0
		{
			self.uses.shrink_to_fit();
			true
		}
		else
		{
			false
		}
	}

	#[inline(always)]
	pub fn numberOfReceiveThenTransmitQueuePairs(&self) -> u16
	{
		self.uses.len() as u16
	}

	#[inline(always)]
	pub fn logicalCore(&self, receiveQueueIdentifier: QueueIdentifier) -> Option<&LogicalCore>
	{
		self.uses.get(receiveQueueIdentifier as usize)
	}

	#[inline(always)]
	pub fn usage(&self) -> (usize, usize, usize, usize)
	{
		(self.wouldLikeToMakeUseOf, self.canMakeUseOf, self.canStillMakeUseOf, self.uses.len())
	}
}
