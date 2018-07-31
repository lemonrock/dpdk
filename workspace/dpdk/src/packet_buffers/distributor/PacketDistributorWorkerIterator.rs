// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Iterates over all workers.
#[derive(Debug)]
pub struct PacketDistributorWorkerIterator
{
	distributor: NonNull<rte_distributor>,
	number_of_workers: u8,
}

impl Iterator for PacketDistributorWorkerIterator
{
	type Item = PacketDistributorWorker;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.number_of_workers == 0
		{
			return None
		}
		
		self.number_of_workers -= 1;
		
		Some
		(
			PacketDistributorWorker
			{
				distributor: self.distributor,
				worker_identifier: self.number_of_workers as u32,
			}
		)
	}
}
