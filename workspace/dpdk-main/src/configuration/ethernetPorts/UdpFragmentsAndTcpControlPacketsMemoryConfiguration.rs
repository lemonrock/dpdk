// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct UdpFragmentsAndTcpControlPacketsMemoryConfiguration
{
	numberOfElements: u32,
	perCoreObjectCacheSize: u32,
}

impl Default for UdpFragmentsAndTcpControlPacketsMemoryConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			numberOfElements: 0x20000, // TLDK main.c MPOOL_NB_BUF,
			perCoreObjectCacheSize: 0x100, // TLDK main.c MPOOL_CACHE_SIZE,
		}
	}
}

impl UdpFragmentsAndTcpControlPacketsMemoryConfiguration
{
	const ApplicationPrivateSize: u16 = 0;
	
	const DataRoomSize: u16 = RTE_PKTMBUF_HEADROOM as u16 + TLE_DST_MAX_HDR as u16; // TLDK main.c FRAG_MBUF_BUF_SIZE
	
	pub fn createPacketBufferPool(&self, ethernetPortIdentifier: EthernetPortIdentifier, queueIdentifier: QueueIdentifier, logicalCoreMemorySocket: Option<NumaSocketId>) -> PacketBufferPool
	{
		let memoryZoneName = format!("UdpTcp-{}-{}", ethernetPortIdentifier, queueIdentifier);
		// MPOOL_NB_BUF, MPOOL_CACHE_SIZE, 0, FRAG_MBUF_BUF_SIZE
		PacketBufferPool::new(&memoryZoneName, self.numberOfElements, self.perCoreObjectCacheSize, Self::ApplicationPrivateSize, Self::DataRoomSize, logicalCoreMemorySocket).expect("Could not allocate a fragment packet buffer pool")
	}
}
