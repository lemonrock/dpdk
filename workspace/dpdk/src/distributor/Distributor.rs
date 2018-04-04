// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distributor(*mut rte_distributor);

impl Drop for Distributor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// The DPDK API provides no explicit way to free or close a distributor. The best we can do is flush.
		self.flushSoThatThereAreNoInFlightOrBackloggedPacketsAwaitingProcessing();
	}
}

unsafe impl Send for Distributor
{
}

unsafe impl Sync for Distributor
{
}

impl Distributor
{
	pub fn create(name: &str, numaSocketId: NumaSocketId, numberOfWorkers: u8) -> Option<(Arc<Self>, Vec<DistributorWorker>)>
	{
		debug_assert!(name.len() < RTE_DISTRIBUTOR_NAMESIZE, "name '{}' is too long, it must be less than '{}'", name, RTE_DISTRIBUTOR_NAMESIZE);
		debug_assert!(numberOfWorkers != 0, "numberOfWorkers can not be zero");
		debug_assert!(numberOfWorkers < Self::RTE_DISTRIB_MAX_WORKERS, "numberOfWorkers '{}' is too long, it must be less than '{}'", name, Self::RTE_DISTRIB_MAX_WORKERS);
		
		let name = CString::new(name).expect("name contained an embedded ASCII NUL");
		
		let x = numaSocketId.as_u8();
		
		let result = unsafe { ::dpdk_sys::rte_distributor_create(name.as_ptr(), x as u32, numberOfWorkers as u32) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOMEM => None,
			
				E::EINVAL => panic!("Supplied an invalid value"),
			
				illegal @ _ => panic!("Unexpected errno '{}' from rte_distributor_create()", illegal),
			}
		}
		else
		{
			let distributor = Arc::new(Distributor(result));
			let numberOfWorkers = numberOfWorkers as usize;
			let mut distributorWorkers = Vec::with_capacity(numberOfWorkers);
			for workerIdentifier in 0..numberOfWorkers
			{
				distributorWorkers.push(DistributorWorker::new(&distributor, workerIdentifier as u8))
			}
			
			Some((distributor, distributorWorkers))
		}
	}
	
	#[inline(always)]
	pub fn as_mut_ptr(&self) -> *mut rte_distributor
	{
		self.0
	}
	
	/// Not thread-safe; only use for one LogicalCore at a time
	#[inline(always)]
	pub fn distributePacketsToWorkersToProcess(&self, packets: &mut [*mut rte_mbuf]) -> u31
	{
		#[cfg(target_pointer_width = "64")] debug_assert!(packets.len() <= ::std::u32::MAX as usize, "Can not process more than 2^32 -1 packets at a time");
		
		match unsafe { ::dpdk_sys::rte_distributor_process(self.0, packets.as_mut_ptr(), packets.len() as u32) }
		{
			numberOfPacketsProcessed if numberOfPacketsProcessed >= 0 => numberOfPacketsProcessed as u31,
			
			invalid @ _ => panic!("The function rte_distributor_process() returned as unexpected value '{}'", invalid),
		}
	}
	
	/// Not thread-safe; call only after calling distributePacketsToWorkersToProcess on the same LogicalCore
	/// It is an error to provide a putPacketsInto with no space available without reallocating, ie capacity() - len() == 0
	/// Returns number of packets actually enqueued (should be <= availableCapacity)
	#[inline(always)]
	pub fn getPacketsReturnedByWorkers(&self, putPacketsInto: &mut Vec<*mut rte_mbuf>) -> usize
	{
		#[cfg(target_pointer_width = "64")] debug_assert!(putPacketsInto.capacity() <= ::std::u32::MAX as usize, "Can not process more than 2^32 -1 packets at a time");
		
		let length = putPacketsInto.len();
		let actualCapacity = putPacketsInto.capacity() - length;
		debug_assert!(actualCapacity != 0, "putPacketsInto has an actualCapacity of 0, ie it is full without reallocating");
				
		match unsafe { ::dpdk_sys::rte_distributor_returned_pkts(self.0, putPacketsInto.as_mut_ptr().offset(length as isize), actualCapacity as u32) }
		{
			numberOfPacketsReturned if numberOfPacketsReturned >= 0 =>
			{
				unsafe { putPacketsInto.set_len(length + numberOfPacketsReturned as usize) };
				numberOfPacketsReturned as usize
			}
			
			invalid @ _ => panic!("The function rte_distributor_process() returned as unexpected value '{}'", invalid),
		}
	}
	
	/// Not thread-safe; call only after calling distributePacketsToWorkersToProcess on the same LogicalCore
	#[inline(always)]
	pub fn flushSoThatThereAreNoInFlightOrBackloggedPacketsAwaitingProcessing(&self) -> u31
	{
		match unsafe { ::dpdk_sys::rte_distributor_flush(self.0) }
		{
			numberOfQueuedOrInFlightPacketsCompleted if numberOfQueuedOrInFlightPacketsCompleted >= 0 => numberOfQueuedOrInFlightPacketsCompleted as u31,
			
			invalid @ _ => panic!("The function rte_distributor_flush() returned as unexpected value '{}'", invalid),
		}
	}
	
	/// Not thread-safe; call only after calling distributePacketsToWorkersToProcess on the same LogicalCore
	#[inline(always)]
	pub fn clearsTheArrayOfReturnedPackets(&self)
	{
		unsafe { ::dpdk_sys::rte_distributor_clear_returns(self.0) };
	}
}

impl Distributor
{
	pub const RTE_DISTRIB_MAX_WORKERS: u8 = 64;
}
