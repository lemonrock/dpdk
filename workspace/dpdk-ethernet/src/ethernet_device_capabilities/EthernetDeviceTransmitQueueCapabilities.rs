// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ethernet device's transmit queue capabilities.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EthernetDeviceTransmitQueueCapabilities
{
	/// Defaults to `TransmitHardwareOffloadingFlags::default()` which is currently the `TransmitHardwareOffloadingFlags::common_flags()`.
	#[serde(default)]
	pub queue_hardware_offloading_flags: TransmitHardwareOffloadingFlags,
	
	/// Queue ring size.
	///
	/// Defaults to `ReceiveQueueRingSize::default()` (currently `RTE_ETH_DEV_FALLBACK_TX_RINGSIZE` which is 512).
	///
	/// If constructed from `rte_eth_dev_info` uses `rte_eth_dev_info.default_txportconf.ring_size` rather than `rte_eth_dev_info.tx_desc_lim.nb_max`.
	#[serde(default)]
	pub queue_ring_size: TransmitQueueRingSize,
	
	/// The ideal number of packets to receive in a 'burst'.
	pub queue_burst_size: NonZeroUsize,
	
	/// Thresholds for packet memory management.
	pub threshold: TransmitRingThresholdRegisters,
	
	/// Threshold for freeing packets.
	pub free_threshold: NonZeroU16,
	
	/// Something Intel specific, called the 'RS' bit.
	pub intel_specific_rs_bit_threshold: u16,
}

impl EthernetDeviceTransmitQueueCapabilities
{
	#[inline(always)]
	pub(crate) fn from(dpdk_information: &rte_eth_dev_info) -> Self
	{
		Self
		{
			queue_hardware_offloading_flags: TransmitHardwareOffloadingFlags::from_bits_truncate(dpdk_information.tx_queue_offload_capa),
			queue_ring_size: ReceiveQueueRingSize(dpdk_information.default_txportconf.ring_size),
			queue_burst_size: dpdk_information.default_txportconf.burst_size as usize,
			threshold: TransmitRingThresholdRegisters::from(dpdk_information.default_txconf.tx_thresh),
			free_threshold: dpdk_information.default_txconf.tx_free_thresh,
			intel_specific_rs_bit_threshold: dpdk_information.default_txconf.tx_rs_thresh,
		}
	}
	
	/// Transmit hardware offloading flags for what the ethernet device supports for a transmit queue.
	#[inline(always)]
	pub fn queue_hardware_offloading_flags(&self) -> TransmitHardwareOffloadingFlags
	{
		self.queue_hardware_offloading_flags
	}
	
	/// Transmit queue ring size.
	#[inline(always)]
	pub fn queue_ring_size(&self, queue_ring_size_constraints: &QueueRingSizeConstraints<TransmitQueueRingSize>) -> TransmitQueueRingSize
	{
		queue_ring_size_constraints.constrain(self.queue_ring_size)
	}
	
	/// Transmit burst maximum packets.
	#[inline(always)]
	pub fn burst_maximum_packets(&self) -> usize
	{
		self.queue_burst_size.get()
	}
	
	/// Transmit threshold.
	#[inline(always)]
	pub fn threshold(&self) -> TransmitRingThresholdRegisters
	{
		self.threshold
	}
	
	/// Transmit free threshold.
	#[inline(always)]
	pub fn free_threshold(&self) -> u16
	{
		self.free_threshold.get()
	}
	
	/// Transmit 'RS' bit threshold.
	///
	/// Only applies to some Intel hardware.
	#[inline(always)]
	pub fn intel_specific_rs_bit_threshold(&self) -> u16
	{
		self.intel_specific_rs_bit_threshold
	}
}
