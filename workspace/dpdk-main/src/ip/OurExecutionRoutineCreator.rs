// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct OurExecutionRoutineCreator
{
	pub ethernetPortConfiguration: ::configuration::ethernetPorts::EthernetPortConfiguration,
	pub ethernetPort: EthernetPort,
	pub arpCaches: Arc<RwLock<HashMap<VirtualLanKey, ArpCache>>>,
	pub queueIdentifier: QueueIdentifier,
	pub logicalCoreMemorySocket: Option<NumaSocketId>,
	pub receiveBurst: ReceiveBurst,
	pub transmitBurst: TransmitBurst,
}

impl ExecutionRoutineCreator for OurExecutionRoutineCreator
{
	fn createExecutionRoutineWhilstExecutingOnSlaveLogicalCore(&mut self) -> Box<ExecutionRoutine>
	{
		let outboundQueue = TransmitBurstQueue::new(self.transmitBurst);

		Box::new(OurExecutionRoutine
		{
			receiveBurst: self.receiveBurst,
			receivedPacketProcessor: ReceivedPacketProcessor
			{
				destinations: self.ethernetPortConfiguration.createPerLogicalCore(self.ethernetPort, self.arpCaches.clone(), self.queueIdentifier, self.logicalCoreMemorySocket),
				outboundQueue,
			},
		})
	}
}
