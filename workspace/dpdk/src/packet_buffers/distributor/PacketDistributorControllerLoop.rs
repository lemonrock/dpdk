// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
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
