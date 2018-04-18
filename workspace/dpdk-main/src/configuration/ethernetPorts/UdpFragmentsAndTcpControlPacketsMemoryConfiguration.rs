// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// UDP fragments and TCP control packets memory configuration.
///
/// Defaults `number_of_elements` to TLDK `main.c` `MPOOL_NB_BUF`.
///
/// Defaults `per_core_object_cache_size` to TLDK `main.c` `MPOOL_CACHE_SIZE`.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct UdpFragmentsAndTcpControlPacketsMemoryConfiguration
{
	number_of_elements: u32,
	per_core_object_cache_size: u32,
}

impl Default for UdpFragmentsAndTcpControlPacketsMemoryConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			number_of_elements: 0x20000,
			per_core_object_cache_size: 0x100,
		}
	}
}

impl UdpFragmentsAndTcpControlPacketsMemoryConfiguration
{
	const ApplicationPrivateSize: u16 = 0;
	
	/// TLDK `main.c` `FRAG_MBUF_BUF_SIZE`.
	const DataRoomSize: u16 = NonNull::<rte_mbuf>::HeadRoom + TLE_DST_MAX_HDR as u16;
	
	pub fn createPacketBufferPool(&self, ethernetPortIdentifier: EthernetPortIdentifier, queueIdentifier: QueueIdentifier, logicalCoreMemorySocket: Option<NumaSocketId>) -> PacketBufferPool
	{
		let memoryZoneName = format!("UdpTcp-{}-{}", ethernetPortIdentifier, queueIdentifier);
		// MPOOL_NB_BUF, MPOOL_CACHE_SIZE, 0, FRAG_MBUF_BUF_SIZE
		PacketBufferPool::new(&memoryZoneName, self.number_of_elements, self.per_core_object_cache_size, Self::ApplicationPrivateSize, Self::DataRoomSize, logicalCoreMemorySocket).expect("Could not allocate a fragment packet buffer pool")
	}
}
