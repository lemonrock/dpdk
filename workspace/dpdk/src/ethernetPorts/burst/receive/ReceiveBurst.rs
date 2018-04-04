// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveBurst
{
	pub function: ReceiveBurstFunction,
	pub data: ReceiveBurstFunctionData,
}

impl ReceiveBurst
{
	pub fn new(ethernetPort: EthernetPort, receiveQueueIdentifier: QueueIdentifier) -> Self
	{
		let underlyingEthernetDevice = ethernetPort.underlyingEthernetDevice();
		
		let data = unsafe
		{
			let ethernetDeviceData = *(underlyingEthernetDevice.data);
			*(ethernetDeviceData.rx_queues.offset(receiveQueueIdentifier as isize))
		};
		
		Self
		{
			function: underlyingEthernetDevice.rx_pkt_burst.unwrap(),
			data: data,
		}
	}
	
	#[inline(always)]
	pub fn receive(&self, queue: *mut *mut rte_mbuf, count: u16) -> u16
	{
		unsafe { (self.function)(self.data, queue, count) }
	}
}
