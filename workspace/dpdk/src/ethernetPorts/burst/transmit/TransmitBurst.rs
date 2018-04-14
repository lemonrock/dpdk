// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmitBurst
{
	function: TransmitBurstFunction,
	data: TransmitBurstFunctionData,
}

impl TransmitBurst
{
	pub fn new(ethernetPort: EthernetPort, transmitQueueIdentifier: QueueIdentifier) -> Self
	{
		let underlying_ethernet_device = ethernetPort.underlying_ethernet_device();

		let data = unsafe
		{
			let ethernetDeviceData = *(underlying_ethernet_device.data);
			*(ethernetDeviceData.tx_queues.offset(transmitQueueIdentifier as isize))
		};

		Self
		{
			function: underlying_ethernet_device.tx_pkt_burst.unwrap(),
			data,
		}
	}

	#[inline(always)]
	pub fn transmit(&self, queue: *mut *mut rte_mbuf, count: usize) -> usize
	{
		let numberTransmitted = unsafe { (self.function)(self.data, queue, count as u16) } as usize;

		debug_assert!(numberTransmitted <= count, "transmitBurstFunction transmitted more '{}' than was possible, '{}'", numberTransmitted, count);

		numberTransmitted
	}
}
