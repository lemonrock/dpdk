// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ethernet device's receive queue capabilities.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EthernetDeviceReceiveQueueCapabilities
{
	receive_queue_hardware_offloading_flags: ReceiveHardwareOffloadingFlags,
	receive_queue_ring_size: ReceiveQueueRingSize,
	receive_queue_burst_size: usize,
	receive_threshold: ReceiveRingThresholdRegisters,
	receive_free_threshold: u16,
}

impl EthernetDeviceReceiveQueueCapabilities
{
	#[inline(always)]
	pub(crate) fn from(dpdk_information: &rte_eth_dev_info) -> Self
	{
		Self
		{
			receive_queue_hardware_offloading_flags: ReceiveHardwareOffloadingFlags::from_bits_truncate(dpdk_information.rx_queue_offload_capa),
			receive_queue_ring_size: ReceiveQueueRingSize(dpdk_information.rx_desc_lim.nb_max),
			receive_queue_burst_size: dpdk_information.default_rxportconf.burst_size as usize,
			receive_threshold: ReceiveRingThresholdRegisters::from(dpdk_information.default_rxconf.rx_thresh),
			receive_free_threshold: dpdk_information.default_rxconf.rx_free_thresh,
		}
	}
	
	/// Receive hardware offloading flags for what the ethernet device supports for a receive queue.
	#[inline(always)]
	pub fn receive_queue_hardware_offloading_flags(&self) -> ReceiveHardwareOffloadingFlags
	{
		self.receive_queue_hardware_offloading_flags
	}
	
	/// Receive threshold.
	#[inline(always)]
	pub fn receive_threshold(&self) -> ReceiveRingThresholdRegisters
	{
		self.receive_threshold
	}
	
	/// Receive free threshold.
	#[inline(always)]
	pub fn receive_free_threshold(&self) -> u16
	{
		self.receive_free_threshold
	}
	
	/// Receive queue ring size.
	#[inline(always)]
	pub fn receive_queue_ring_size(&self) -> ReceiveQueueRingSize
	{
		self.receive_queue_ring_size
	}
	
	/// Receive burst maximum packets.
	#[inline(always)]
	pub fn receive_burst_maximum_packets(&self) -> usize
	{
		self.receive_queue_burst_size
	}
}
