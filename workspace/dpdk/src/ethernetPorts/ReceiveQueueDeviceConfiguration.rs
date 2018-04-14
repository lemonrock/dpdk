// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveQueueDeviceConfiguration
{
	ringPrefetchThreshold: Option<u8>,
	ringHostThreshold: Option<u8>,
	ringWritebackThreshold: Option<u8>,
	startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis: Option<u16>,
	dropPacketsIfNoDescriptorsAreAvailable: Option<bool>,
	startQueueWhenEthernetDeviceStarted: bool,
}

impl ReceiveQueueDeviceConfiguration
{
	#[inline(always)]
	pub fn new
	(
		ringPrefetchThreshold: Option<u8>,
		ringHostThreshold: Option<u8>,
		ringWritebackThreshold: Option<u8>,
		startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis: Option<u16>,
		dropPacketsIfNoDescriptorsAreAvailable: Option<bool>,
		startQueueWhenEthernetDeviceStarted: bool
	) -> Self
	{
		ReceiveQueueDeviceConfiguration
		{
			ringPrefetchThreshold,
			ringHostThreshold,
			ringWritebackThreshold,
			startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis,
			dropPacketsIfNoDescriptorsAreAvailable,
			startQueueWhenEthernetDeviceStarted,
		}
	}

	#[inline(always)]
	pub fn overrideDropPacketsIfNoDescriptorsAreAvailable() -> Self
	{
		Self::new(None, None, None, None, Some(true), true)
	}

	#[inline(always)]
	pub fn as_rte_eth_rxconf(&self, mut configuration: rte_eth_rxconf) -> rte_eth_rxconf
	{
		if let Some(ringPrefetchThreshold) = self.ringPrefetchThreshold
		{
			configuration.rx_thresh.pthresh = ringPrefetchThreshold;
		}

		if let Some(ringHostThreshold) = self.ringHostThreshold
		{
			configuration.rx_thresh.hthresh = ringHostThreshold;
		}

		if let Some(ringWritebackThreshold) = self.ringWritebackThreshold
		{
			configuration.rx_thresh.wthresh = ringWritebackThreshold;
		}

		if let Some(startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis) = self.startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis
		{
			configuration.rx_free_thresh = startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis;
		}

		if let Some(startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis) = self.startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis
		{
			configuration.rx_free_thresh = startFreeingReceiveBuffersIfThereAreLessFreeDescriptorsThanThis;
		}

		if let Some(dropPacketsIfNoDescriptorsAreAvailable) = self.dropPacketsIfNoDescriptorsAreAvailable
		{
			configuration.rx_drop_en = if dropPacketsIfNoDescriptorsAreAvailable
			{
				1
			}
			else
			{
				0
			};
		}

		configuration.rx_deferred_start = if self.startQueueWhenEthernetDeviceStarted
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
