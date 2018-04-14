// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EthernetPortReceiveModeConfiguration
{
	pub multiQueuePacketDistributionMode: MultiQueuePacketReceiveMode,
	pub maximumReceivePacketLengthOnlyUsedIfJumboFramesEnabled: Option<u16>, // => implies jumbo frames enabled for us
	pub splitHeaderSizeIfHeaderSplitEnabled: Option<u16>, // => implies header split enabled

	pub hardwareIpUdpOrTcpChecksumOffloadEnabled: bool,
	pub hardwareVlanFilterEnabled: bool,
	pub hardwareVlanStripEnabled: bool,
	pub extendedVlanEnabled: bool,
	pub hardwareCyclicRedundancyChecksumStrippingEnabled: bool,
	pub scatterPacketsReceiveHandlerEnabled: bool,
	pub largeReceiveOffloadEnabled: bool,
}

impl Default for EthernetPortReceiveModeConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::defaultish(MaximumTransmissionUnitSizeInBytes::EthernetV2)
	}
}

impl EthernetPortReceiveModeConfiguration
{
	#[inline(always)]
	pub fn defaultish(maximumTransmissionUnitSizeInBytes: MaximumTransmissionUnitSizeInBytes) -> EthernetPortReceiveModeConfiguration
	{
		EthernetPortReceiveModeConfiguration
		{
			multiQueuePacketDistributionMode: MultiQueuePacketReceiveMode::default(),
			maximumReceivePacketLengthOnlyUsedIfJumboFramesEnabled: maximumTransmissionUnitSizeInBytes.conservativeJumboFrameLength(),
			splitHeaderSizeIfHeaderSplitEnabled: None,
			hardwareIpUdpOrTcpChecksumOffloadEnabled: true,
			hardwareVlanFilterEnabled: false,
			hardwareVlanStripEnabled: false,
			extendedVlanEnabled: false,
			hardwareCyclicRedundancyChecksumStrippingEnabled: true,
			scatterPacketsReceiveHandlerEnabled: false,
			largeReceiveOffloadEnabled: true,
		}
	}

	#[inline(always)]
	pub fn disableReceiveSideScaling(&mut self)
	{
		self.multiQueuePacketDistributionMode.disableReceiveSideScaling();
	}

	#[inline(always)]
	pub fn enableReceiveSideScaling(&mut self)
	{
		self.multiQueuePacketDistributionMode.enableReceiveSideScaling();
	}

	#[inline(always)]
	pub fn enableTcpLargeReceiveOffload(&mut self)
	{
		self.largeReceiveOffloadEnabled = true;
	}

	#[inline(always)]
	pub fn disableTcpLargeReceiveOffload(&mut self)
	{
		self.largeReceiveOffloadEnabled = false;
	}

	#[inline(always)]
	pub fn enableHardwareVlanStripping(&mut self)
	{
		self.hardwareVlanStripEnabled = true;
	}

	#[inline(always)]
	pub fn disableHardwareVlanStripping(&mut self)
	{
		self.hardwareVlanStripEnabled = false;
	}

	#[inline(always)]
	pub fn enableHardwareCyclicRedundancyChecksumStripping(&mut self)
	{
		self.hardwareCyclicRedundancyChecksumStrippingEnabled = true;
	}

	#[inline(always)]
	pub fn enableIpV4TcpAndUdpChecksumOffload(&mut self)
	{
		self.hardwareIpUdpOrTcpChecksumOffloadEnabled = true;
	}

	#[inline(always)]
	pub fn disableIpV4TcpAndUdpChecksumOffload(&mut self)
	{
		self.hardwareIpUdpOrTcpChecksumOffloadEnabled = false;
	}

	#[inline(always)]
	pub fn as_rte_eth_rxmode(&self) -> rte_eth_rxmode
	{
		let actualMaximumReceivePacketLength = match self.maximumReceivePacketLengthOnlyUsedIfJumboFramesEnabled
		{
			None => ETHER_MAX_LEN as u32,
			Some(size) =>
			{
				debug_assert!(size <= ETHER_MAX_JUMBO_FRAME_LEN, "Jumbo frame size must be less than or equal to '{}', and can not be '{}'", ETHER_MAX_JUMBO_FRAME_LEN, size);
				size as u32
			}
		};
		debug_assert!(actualMaximumReceivePacketLength as usize >= ETHER_MIN_LEN, "Frame size must be greater than or equal to than '{}', and can not be '{}'", ETHER_MIN_LEN, actualMaximumReceivePacketLength);

		let splitHeaderSize = self.splitHeaderSizeIfHeaderSplitEnabled.unwrap_or(0);

		let mut rxmode = rte_eth_rxmode
		{
			mq_mode: self.multiQueuePacketDistributionMode.as_rte_eth_rx_mq_mode(),
			max_rx_pkt_len: actualMaximumReceivePacketLength,
			split_hdr_size: splitHeaderSize,
			__bindgen_bitfield: 0,
		};

		unsafe
		{
			rust_rte_eth_rxmode_setBitFields
			(
				&mut rxmode,
				self.splitHeaderSizeIfHeaderSplitEnabled.is_some(),
				self.hardwareIpUdpOrTcpChecksumOffloadEnabled,
				self.hardwareVlanFilterEnabled,
				self.hardwareVlanStripEnabled,
				self.extendedVlanEnabled,
				self.maximumReceivePacketLengthOnlyUsedIfJumboFramesEnabled.is_some(),
				self.hardwareCyclicRedundancyChecksumStrippingEnabled,
				self.scatterPacketsReceiveHandlerEnabled,
				self.largeReceiveOffloadEnabled
			)
		}

		rxmode
	}
}
