// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct OurExecutionRoutine
{
	pub receiveBurst: ReceiveBurst,
	pub receivedPacketProcessor: ReceivedPacketProcessor,
}

impl ExecutionRoutine for OurExecutionRoutine
{
	#[inline(always)]
	fn start(&mut self)
	{
	}

	#[inline(always)]
	fn execute(&mut self) -> bool
	{
		{
			const MaximumReceivePacketBurst: usize = 32;
			const MaximumReceivePacketBurstU16: u16 = MaximumReceivePacketBurst as u16;

			let mut packets: [*mut rte_mbuf; MaximumReceivePacketBurst] = unsafe { uninitialized() };
			let numberOfPacketsRetrieved = self.receiveBurst.receive(packets.as_mut_ptr(), MaximumReceivePacketBurstU16);
			debug_assert!(numberOfPacketsRetrieved <= MaximumReceivePacketBurstU16, "Violation of receive contract");

			for index in 0..(numberOfPacketsRetrieved as usize)
			{
				let packet = unsafe { *packets.get_unchecked(index) };
				self.receivedPacketProcessor.processPacket(packet);
			}
			forget(packets);

			// self.receivedPacketProcessor.send();

			// sort incoming packets into queues
		}

		// N

		// Loop over inbound queues - use an array vec? - or over the hash map keys in recv'd packet processor
		// Call TCP device, bulk send to TLDK, poke context, bulk send from TLDK
		// Call UDP device, bulk send to TLDK, (no need to), bulk send from TLDK

		// Handle any streams








		false
	}

	#[inline(always)]
	fn stop(&mut self)
	{
	}
}


// TODO: Copy logic from Device
// TODO: Have an inboundQueue per vlan, per internet_protocol address, per tcp / udp context, ie per IpAddressInformation
// TODO: Sort incoming packets into those queues, then loop over devices for those queues, process
