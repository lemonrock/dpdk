// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveTransmitQueueMemoryConfiguration
{
	packetBuffersPerReceiveQueue: Option<u32>,
	packetBuffersPerTransmitQueue: Option<u32>,
	perCoreObjectCacheSize: u32,
	applicationPrivateSize: u16,
	dataRoomSize: u16
}

impl Default for ReceiveTransmitQueueMemoryConfiguration
{
	fn default() -> Self
	{
		assert!(Self::PacketBufferDataSize >= NonNull::<rte_mbuf>::DefaultDataRoom, "PacketBufferDataSize '{}' is less than the minimum, NonNull::<rte_mbuf>::DefaultDataRoom '{}'", Self::PacketBufferDataSize, NonNull::<rte_mbuf>::DefaultDataRoom);

		let dataRoomSize = buffer_length(Self::PacketBufferDataSize);

		ReceiveTransmitQueueMemoryConfiguration::new(None, None, Self::CacheSize, Self::ApplicationPrivateSize, dataRoomSize)
	}
}

impl QueueMemoryConfiguration for ReceiveTransmitQueueMemoryConfiguration
{
	#[inline(always)]
	fn receiveQueueDescriptorsDmaMemoryAllocatedFromNumaSocketIdAndMemoryPool(&self, ethernetPortInformation: &EthernetPortInformation, receiveQueueIdentifier: QueueIdentifier, numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize: u16) -> (Option<NumaSocketId>, PacketBufferPool)
	{
		self.create(ethernetPortInformation, receiveQueueIdentifier, numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize, self.packetBuffersPerReceiveQueue, "RxQ")
	}

	#[inline(always)]
	fn transmitQueueDescriptorsDmaMemoryAllocatedFromNumaSocketIdAndMemoryPool(&self, ethernetPortInformation: &EthernetPortInformation, transmitQueueIdentifier: QueueIdentifier, numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize: u16) -> (Option<NumaSocketId>, PacketBufferPool)
	{
		self.create(ethernetPortInformation, transmitQueueIdentifier, numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize, self.packetBuffersPerTransmitQueue, "TxQ")
	}
}

impl ReceiveTransmitQueueMemoryConfiguration
{
	pub const DefaultPacketBufferDataSize: u16 = NonNull::<rte_mbuf>::DefaultDataRoom;
	
	pub const DefaultDataRoomSize: u16 = NonNull::<rte_mbuf>::DefaultBufferLength;

	#[inline(always)]
	pub fn new(packetBuffersPerReceiveQueue: Option<u32>, packetBuffersPerTransmitQueue: Option<u32>, perCoreObjectCacheSize: u32, applicationPrivateSize: u16, dataRoomSize: u16) -> Self
	{
		assert!(perCoreObjectCacheSize <= RTE_MEMPOOL_CACHE_MAX_SIZE, "perCoreObjectCacheSize '{}' exceeds RTE_MEMPOOL_CACHE_MAX_SIZE '{}'", perCoreObjectCacheSize, RTE_MEMPOOL_CACHE_MAX_SIZE);
		assert!(dataRoomSize >= NonNull::<rte_mbuf>::DefaultBufferLength, "dataRoomSize '{}' is less than the minimum, NonNull::<rte_mbuf>::DefaultBufferLength '{}'", dataRoomSize, NonNull::<rte_mbuf>::DefaultBufferLength);

		ReceiveTransmitQueueMemoryConfiguration
		{
			packetBuffersPerReceiveQueue,
			packetBuffersPerTransmitQueue,
			perCoreObjectCacheSize,
			applicationPrivateSize,
			dataRoomSize,
		}
	}

	#[inline(always)]
	fn create(&self, ethernetPortInformation: &EthernetPortInformation, queueIdentifier: u16, ringSize: u16, packetBuffersPerQueueOverride: Option<u32>, queueNamePrefix: &'static str) -> (Option<NumaSocketId>, PacketBufferPool)
	{
		let portIdentifier = ethernetPortInformation.portIdentifier();

		let memoryZoneName = format!("{}-{}-{}", queueNamePrefix.to_owned(), portIdentifier, queueIdentifier);

		let packetBuffersPerQueue = match packetBuffersPerQueueOverride
		{
			Some(value) => value,

			// Use double the ring size because of the way some of the drivers refill the ring queue
			// Optimum value is numberOfElements = (2^n - 1), where n is a power of 2, eg 4095, 65,535, etc
			None => 2 * ringSize as u32,
		};

		let numa_socket_id = ethernetPortInformation.logicalCoreFor(queueIdentifier).optionalNumaSocketId();

		let packetBufferPool = PacketBufferPool::new(&memoryZoneName, packetBuffersPerQueue, self.perCoreObjectCacheSize, self.applicationPrivateSize, self.dataRoomSize, numa_socket_id);

		(ethernetPortInformation.ethernetPort().parentNumaSocketId(), packetBufferPool.expect("Not enough memory to create PacketBufferPool for queue"))
	}
}
