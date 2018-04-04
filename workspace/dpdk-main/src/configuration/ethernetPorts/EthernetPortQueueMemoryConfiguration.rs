// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct EthernetPortQueueMemoryConfiguration
{
	packetBuffersPerReceiveQueue: Option<u32>,
	packetBuffersPerTransmitQueue: Option<u32>,
	perCoreObjectCacheSize: u32,
}

impl Default for EthernetPortQueueMemoryConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			packetBuffersPerReceiveQueue: Some(0x20000), // TLDK l4fwd example MPOOL_NB_BUF
			packetBuffersPerTransmitQueue: Some(0x20000), // TLDK l4fwd example MPOOL_NB_BUF
			perCoreObjectCacheSize: 0x100, // TLDK l4fwd example MPOOL_CACHE_SIZE
		}
	}
}

impl EthernetPortQueueMemoryConfiguration
{
	pub fn receiveTransmitQueueMemoryConfiguration(&self) -> ReceiveTransmitQueueMemoryConfiguration
	{
		// This logic has a memory pool per ethernet device, per queue (and thus per logical core), per Receive or Transmit Queue
		// TLDK instead (port.h, netbe_port_init() call to pool_init()) has a memory pool per NUMA socket [found from logical core queue pair will be on], shared across ethernet devices, shared across Receive or Transmit Queue
		// TODO: Our design here has the potential to explode memory usage by a factor of eight or more (in new designs with many cores per socket)
		// eg AMD Naples could be 2-socket, with 64 hardware threads (logical cores) per socket
		ReceiveTransmitQueueMemoryConfiguration::new
		(
			self.packetBuffersPerReceiveQueue,
			self.packetBuffersPerTransmitQueue,
			self.perCoreObjectCacheSize,
			0,
			ReceiveTransmitQueueMemoryConfiguration::DefaultDataRoomSize
		)
	}
}
