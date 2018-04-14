// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const MaximumTransmitQueues: usize = RTE_MAX_QUEUES_PER_PORT;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmitQueueConfiguration
{
	pub numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize: u16,
	pub overrideDefaultDeviceConfiguration: Option<TransmitQueueDeviceConfiguration>,
	pub maximumTransmissionRateInMbps: Option<u16>,
}

impl Default for TransmitQueueConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(Self::DefaultNumberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize, None, None)
	}
}

impl TransmitQueueConfiguration
{
	pub const DefaultNumberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize: u16 = 512;

	pub const TldkNumberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize: u16 = 2048; // TLDK's l4fwd app's TX_RING_SIZE

	pub fn new(numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize: u16, overrideDefaultDeviceConfiguration: Option<TransmitQueueDeviceConfiguration>, maximumTransmissionRateInMbps: Option<u16>) -> Self
	{
		if let Some(overrideDefaultDeviceConfiguration) = overrideDefaultDeviceConfiguration
		{
			overrideDefaultDeviceConfiguration.validate(numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize);
		}

		TransmitQueueConfiguration
		{
			numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize,
			overrideDefaultDeviceConfiguration,
			maximumTransmissionRateInMbps,
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
