// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait QueueMemoryConfiguration
{
	const CacheSize: u32 = RTE_MEMPOOL_CACHE_MAX_SIZE; // 512; consider making the numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize / numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize the same value
	const PacketBufferDataSize: u16 = 2048;
	const ApplicationPrivateSize: u16 = 0;
	
	#[inline(always)]
	fn receiveQueueDescriptorsDmaMemoryAllocatedFromNumaSocketIdAndMemoryPool(&self, ethernetPortInformation: &EthernetPortInformation, receiveQueueIdentifier: QueueIdentifier, numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize: u16) -> (Option<NumaSocketId>, PacketBufferPool);
	
	#[inline(always)]
	fn transmitQueueDescriptorsDmaMemoryAllocatedFromNumaSocketIdAndMemoryPool(&self, ethernetPortInformation: &EthernetPortInformation, transmitQueueIdentifier: QueueIdentifier, numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize: u16) -> (Option<NumaSocketId>, PacketBufferPool);
}
