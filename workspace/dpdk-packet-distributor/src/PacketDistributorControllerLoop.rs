// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Use this to manage a packet distributor controller.
///
/// This particular implementation ***DOES NOT*** process returned packets.
pub struct PacketDistributorControllerLoop
{
	should_function_terminate: Arc<ShouldFunctionTerminate>,
	packet_distributor_controller: PacketDistributorController,
	ring_buffer_consumer: RingBufferConsumer<NonNull<rte_mbuf>>,
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
	/// Use this to create a new instance.
	#[inline(always)]
	pub fn new(should_function_terminate: &Arc<ShouldFunctionTerminate>, packet_distributor_controller: PacketDistributorController, ring_buffer_consumer: RingBufferConsumer<NonNull<rte_mbuf>>) -> Self
	{
		Self
		{
			should_function_terminate: should_function_terminate.clone(),
			packet_distributor_controller,
			ring_buffer_consumer,
		}
	}
	
	#[inline(always)]
	fn execute_once(&self)
	{
		self.distribute();
		self.gather_and_free();
		
	}
	
	#[inline(always)]
	fn should_continue(&self) -> bool
	{
		self.should_function_terminate.should_continue()
	}
	
	#[inline(always)]
	fn distribute(&self)
	{
		let ring_buffer_consumer_guard = self.ring_buffer_consumer.consume();
		let packets_from = ring_buffer_consumer_guard.buffer_slice;
		let _distributed = self.packet_distributor_controller.distribute_packets_to_workers_slice(packets_from);
	}
	
	#[inline(always)]
	fn gather_and_free(&self)
	{
		while
		{
			let mut packets_into: ArrayVec<[NonNull<rte_mbuf>; 32]> = ArrayVec::new();
			let gathered = self.packet_distributor_controller.gather_packets_from_workers(&mut packets_into);
			for packet in packets_into.iter()
			{
				unsafe { rust_rte_pktmbuf_free(packet.as_ptr()) }
			}
			gathered
		}
		{
		}
	}
}
