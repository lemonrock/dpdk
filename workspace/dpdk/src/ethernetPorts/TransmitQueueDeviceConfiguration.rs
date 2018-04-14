// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmitQueueDeviceConfiguration
{
	ringPrefetchThreshold: Option<u8>,
	ringHostThreshold: Option<u8>,
	ringWritebackThreshold: Option<u8>,
	startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis: Option<u16>,
	transmitDescriptorsRSbitThreshold: Option<u16>,
	flagsToRemove: TransmitQueueFlags,
	flagsToInsert: TransmitQueueFlags,
	startQueueWhenEthernetDeviceStarted: bool,
}

impl TransmitQueueDeviceConfiguration
{
	#[inline(always)]
	pub fn new
	(
		ringPrefetchThreshold: Option<u8>,
		ringHostThreshold: Option<u8>,
		ringWritebackThreshold: Option<u8>,
		startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis: Option<u16>,
		transmitDescriptorsRSbitThreshold: Option<u16>,
		flagsToRemove: TransmitQueueFlags,
		flagsToInsert: TransmitQueueFlags,
		startQueueWhenEthernetDeviceStarted: bool
	) -> Self
	{
		if let Some(ringWritebackThreshold) = ringWritebackThreshold
		{
			if ringWritebackThreshold > 0
			{
				if let Some(transmitDescriptorsRSbitThreshold) = transmitDescriptorsRSbitThreshold
				{
					assert_eq!(transmitDescriptorsRSbitThreshold, 1, "if ringWritebackThreshold '{}' is greater than zero, then transmitDescriptorsRSbitThreshold '{}' must be one", ringWritebackThreshold, transmitDescriptorsRSbitThreshold);
				}
			}
		}

		TransmitQueueDeviceConfiguration
		{
			ringPrefetchThreshold,
			ringHostThreshold,
			ringWritebackThreshold,
			transmitDescriptorsRSbitThreshold,
			startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis,
			flagsToRemove,
			flagsToInsert,
			startQueueWhenEthernetDeviceStarted,
		}
	}

	#[inline(always)]
	pub fn overrideForTldk(startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis: u16) -> Self
	{
		Self::new(None, None, None, Some(startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis), None, TransmitQueueFlags::all(), TransmitQueueFlags::empty(), true)
	}

	#[inline(always)]
	pub fn validate(&self, numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize: u16)
	{
		if let Some(transmitDescriptorsRSbitThreshold) = self.transmitDescriptorsRSbitThreshold
		{
			assert!(transmitDescriptorsRSbitThreshold < numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize - 3, "transmitDescriptorsRSbitThreshold '{}' is too large for numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize '{}' - 3", transmitDescriptorsRSbitThreshold, numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize);
		}

		if let Some(startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis) = self.startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis
		{
			assert!(startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis < numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize - 3, "startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis '{}' is too large for numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize '{}' - 3", startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis, numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize);
		}
	}

	#[inline(always)]
	pub fn as_rte_eth_txconf(&self, mut configuration: rte_eth_txconf) -> rte_eth_txconf
	{
		if let Some(ringPrefetchThreshold) = self.ringPrefetchThreshold
		{
			configuration.tx_thresh.pthresh = ringPrefetchThreshold;
		}

		if let Some(ringHostThreshold) = self.ringHostThreshold
		{
			configuration.tx_thresh.hthresh = ringHostThreshold;
		}

		if let Some(ringWritebackThreshold) = self.ringWritebackThreshold
		{
			configuration.tx_thresh.wthresh = ringWritebackThreshold;
		}

		if let Some(startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis) = self.startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis
		{
			configuration.tx_free_thresh = startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis;
		}

		if let Some(transmitDescriptorsRSbitThreshold) = self.transmitDescriptorsRSbitThreshold
		{
			configuration.tx_rs_thresh = transmitDescriptorsRSbitThreshold;
		}

		let mut flags = TransmitQueueFlags::from_bits_truncate(configuration.txq_flags);
		flags.remove(self.flagsToRemove);
		flags.insert(self.flagsToInsert);
		configuration.txq_flags = flags.bits();

		configuration.tx_deferred_start = if self.startQueueWhenEthernetDeviceStarted
		{
			0
		}
		else
		{
			1
		};

		configuration
	}

	#[inline(always)]
	pub fn startQueueWhenEthernetDeviceStarted(&self) -> bool
	{
		self.startQueueWhenEthernetDeviceStarted
	}
}
