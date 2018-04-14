// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const MaximumReceiveQueues: usize = RTE_MAX_QUEUES_PER_PORT;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveQueueConfiguration
{
	pub numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize: u16,
	pub overrideDefaultDeviceConfiguration: Option<ReceiveQueueDeviceConfiguration>,
	pub enableVlanStripping: Option<bool>,
}

impl Default for ReceiveQueueConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		ReceiveQueueConfiguration
		{
			numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize: Self::DefaultNumberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize,
			overrideDefaultDeviceConfiguration: None,
			enableVlanStripping: None,
		}
	}
}

impl ReceiveQueueConfiguration
{
	pub const DefaultNumberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize: u16 = 512;

	pub const TldkNumberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize: u16 = 1024; // TLDK l4fwd app's RX_RING_SIZE

	#[inline(always)]
	pub fn new(numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize: u16, overrideDefaultDeviceConfiguration: Option<ReceiveQueueDeviceConfiguration>, enableVlanStripping: Option<bool>) -> Self
	{
		ReceiveQueueConfiguration
		{
			numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize,
			overrideDefaultDeviceConfiguration,
			enableVlanStripping,
		}
	}

	#[inline(always)]
	pub fn startQueueWhenEthernetDeviceStarted(&self) -> bool
	{
		if self.overrideDefaultDeviceConfiguration.is_some()
		{
			self.overrideDefaultDeviceConfiguration.unwrap().startQueueWhenEthernetDeviceStarted
		}
		else
		{
			true
		}
	}
}
