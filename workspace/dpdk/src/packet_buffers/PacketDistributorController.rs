// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



pub struct PacketDistributorControllerLoop
{
	should_function_terminate: Arc<ShouldFunctionTerminate>,
	packet_distributor_controller: PacketDistributorController,
	ring_buffer_consumer: RingBufferConsumer,
}

impl SlaveLogicalCoreFunction for PacketDistributorControllerLoop
{
	#[inline(always)]
	fn execute(&mut self)
	{
		while self.should_continue()
		{
			self.execute_once()
		}
	}
}

impl ServiceFunction for PacketDistributorControllerLoop
{
	/// Called repeatedly by a service core.
	#[inline(always)]
	fn execute(&mut self)
	{
		if self.should_continue()
		{
			self.execute_once()
		}
	}
}

impl PacketDistributorControllerLoop
{
	#[inline(always)]
	fn execute_once(&self)
	{
		{
			let ring_buffer_consumer_guard = self.ring_buffer_consumer.consume();
			
			XXXX
			// except this isn't an array or vec but a slice...
			let packets_from = ring_buffer_consumer_guard.buffer_slice;
			let packets_start_from_index = 0;
			
			
			self.packet_distributor_controller.distribute_packets_to_workers(packets_from: &mut A, packets_start_from_index);
		}
		
		
		
		XXXX: Where are we sending them to? How expensive is tx_burst? Do we actually need to gather at all?
		
		self.packet_distributor_controller.gather_packets_from_workers(packets_into: &mut A);
		// send these packets somewhere; either on a queue, or just tx-them directly?
		// if can't enqueue them then we free the packets.
		
		
		XXXX: When do we call self.packet_distributor_controller.flush()?
	}
	
	#[inline(always)]
	fn should_continue(&self) -> bool
	{
		self.should_function_terminate.should_continue()
	}
}

/// A packet distributor controller.
///
/// Once created, it can not be destroyed and so its memory can not be reclaimed.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PacketDistributorController
{
	distributor: NonNull<rte_distributor>,
}

impl PacketDistributorController
{
	/// `name` needs to be unique.
	///
	/// `numa_numa_choice` should ideally be for the NUMA socket for the logical core that will use this distributor.
	///
	/// `number_of_workers` can not exceed 64.
	#[inline(always)]
	pub fn new(name: &str, numa_numa_choice: NumaNodeChoice, number_of_workers: u8) -> (Self, PacketDistributorWorkerIterator)
	{
		const RTE_DISTRIBUTOR_NAMESIZE: usize = 32;
		debug_assert!(name.len() < RTE_DISTRIBUTOR_NAMESIZE, "name '{}' is longer than RTE_DISTRIBUTOR_NAMESIZE '{}'", name, RTE_DISTRIBUTOR_NAMESIZE);
		
		const RTE_DISTRIB_MAX_WORKERS: u8 = 64;
		debug_assert!(number_of_workers < RTE_DISTRIB_MAX_WORKERS, "number_of_workers '{}' exceeds RTE_DISTRIB_MAX_WORKERS '{}'", number_of_workers, RTE_DISTRIB_MAX_WORKERS);
		
		let name = CString::new(name).unwrap();
		
		let distributor = unsafe { rte_distributor_create(name.as_ptr(), numa_numa_choice.into(), number_of_workers as u32, rte_distributor_alg_type::RTE_DIST_ALG_BURST as u32) };
		debug_assert!(distributor.is_not_null(), "distributor is null");
		
		let distributor = unsafe { NonNull::new_unchecked(distributor) };
		
		(
			PacketDistributorController
			{
				distributor,
			},
			PacketDistributorWorkerIterator
			{
				distributor,
				number_of_workers,
			}
		)
	}
	
	/// Distributes (received) packets to workers.
	///
	/// Make sure the rss tag is set; the lowest 15-bits ('flow id') are used.
	///
	/// Returns next `packets_start_from_index`; if this is the same as `packets_from.len()` then all packets have been distributed.
	#[inline(always)]
	pub fn distribute_packets_to_workers<A: UnifiedArrayVecAndVec<NonNull<rte_mbuf>>>(&self, packets_from: &mut A, packets_start_from_index: usize) -> usize
	{
		let (receive_packets_from_pointer, number_of_packets_to_receive) = packets_from.to_ffi_data_u32(packets_start_from_index);
		
		let result = unsafe { rte_distributor_process(self.handle(), receive_packets_from_pointer, number_of_packets_to_receive) };
		debug_assert!(result >= 0, "result '{}' was negative", result);
		packets_start_from_index + result as usize
	}
	
	/// Gathers (to transmit) packets from workers.
	///
	/// Returns true if gathered some packets.
	#[inline(always)]
	pub fn gather_packets_from_workers<A: UnifiedArrayVecAndVec<NonNull<rte_mbuf>>>(&self, packets_into: &mut A) -> bool
	{
		let (pointer, number_of_packets) = packets_into.from_ffi_data_u32();
		
		let number_of_packets_gathered_by_us = unsafe { rte_distributor_returned_pkts(self.handle(), pointer, number_of_packets) };
		debug_assert!(number_of_packets_gathered_by_us >= 0, "number_of_packets_gathered_by_us '{}' was negative", number_of_packets_gathered_by_us);
		let number_of_packets_gathered_by_us = number_of_packets_gathered_by_us as usize;
		
		if likely(number_of_packets_gathered_by_us > 0)
		{
			number_of_packets.set_length(number_of_packets_gathered_by_us as usize);
			true
		}
		else
		{
			false
		}
	}
	
	/// Flush the distributor, so that there are no outstanding packets in flight or queued up.
	///
	/// Sends an empty burst after flushing to all workers; they can use this as a signal to exit.
	///
	/// Returns number of packets flushed.
	#[inline(always)]
	pub fn flush(&self) -> usize
	{
		let flushed = unsafe { rte_distributor_flush(self.handle()) };
		debug_assert!(flushed >= 0, "flushed '{}' was negative", flushed);
		flushed as usize
	}
	
	/// ?
	#[inline(always)]
	pub fn clear_gather_packets_buffer(&self)
	{
		unsafe { rte_distributor_clear_returns(self.handle()) }
	}
	
	#[inline(always)]
	fn handle(self) -> *mut rte_distributor
	{
		self.0.as_ptr()
	}
}
