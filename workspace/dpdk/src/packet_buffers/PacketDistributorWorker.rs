// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A packet distributor worker.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PacketDistributorWorker
{
	distributor: NonNull<rte_distributor>,
	worker_identifier: u32,
}

unsafe impl Send for PacketDistributorWorker
{
}

unsafe impl Sync for PacketDistributorWorker
{
}

impl Worker
{
	/// Maximum number of packets that can be received at once.
	pub const MaximumNumberOfPacketsThatCanBeReceivedAtOnce: usize = 8;
	
	/// To be used on different thread to the controller.
	///
	/// Combines `rte_distributor_request_pkt` and `rte_distributor_poll_pkt`.
	///
	/// Returns true if packets were received.
	#[inline(always)]
	pub fn exchange_packets<A: UnifiedArrayVecAndVec<NonNull<rte_mbuf>>>(&self, transmit_packets_from: &mut A, transmit_packets_start_from_index: usize, receive_packets_into: &mut ArrayVec<[NonNull<rte_mbuf>; Self::MaximumNumberOfPacketsThatCanBeReceivedAtOnce]>) -> bool
	{
		debug_assert_eq!(receive_packets_into.length(), 0, "received_packets must be empty");
		let receive_packets_into_pointer = unsafe { transmute(receive_packets_into.as_mut_ptr()) };
		
		let (transmit_packets_from_pointer, number_of_packets_to_transmit) = transmit_packets_from.to_ffi_data_u32(transmit_packets_start_from_index);
		
		let number_of_packets_distributed_to_us = unsafe { rte_distributor_get_pkt(self.handle(), self.worker_identifier, receive_packets_into_pointer, return_packets_from_pointer, number_of_packets_to_transmit) };
		if likely(number_of_packets_distributed_to_us >= 0)
		{
			let number_of_packets_distributed_to_us = number_of_packets_distributed_to_us as usize;
			debug_assert!(number_of_packets_distributed_to_us <= Self::MaximumNumberOfPacketsThatCanBeReceivedAtOnce, "number_of_packets_distributed_to_us '{}' exceeds MaximumNumberOfPacketsThatCanBeReceivedAtOnce '{}'", number_of_packets_distributed_to_us, Self::MaximumNumberOfPacketsThatCanBeReceivedAtOnce);
			
			if likely(number_of_packets_distributed_to_us > 0)
			{
				receive_packets_into.set_length(number_of_packets_distributed_to_us);
				true
			}
			else
			{
				false
			}
		}
		else
		{
			const GaveUpAfterSeveralPollAttempts: i32 = -1;
			debug_assert_eq!(number_of_packets_distributed_to_us, GaveUpAfterSeveralPollAttempts, "number_of_packets_distributed_to_us '{}' was not GaveUpAfterSeveralPollAttempts '{}'", number_of_packets_distributed_to_us, GaveUpAfterSeveralPollAttempts);
			false
		}
	}
	
	/// Should be called just before shutdown.
	#[inline(always)]
	pub fn transmit_remaining_packets_before_shutdown<A: UnifiedArrayVecAndVec<NonNull<rte_mbuf>>>(&self, transmit_packets_from: &mut A)
	{
		const start_from_index: usize = 0;
		let (transmit_packets_from_pointer, number_of_packets_to_transmit) = transmit_packets_from.to_ffi_data_u32(start_from_index);
		
		let result = unsafe { rte_distributor_return_pkt(self.handle(), self.worker_identifier, transmit_packets_from_pointer, number_of_packets_to_transmit) };
		debug_assert_eq!(result, 0, "rte_distributor_return_pkt failed");
	}
	
	#[inline(always)]
	fn handle(self) -> *mut rte_distributor
	{
		self.0.as_ptr()
	}
}
