// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ethernet device information.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EthernetDeviceCapabilities(rte_eth_dev_info);

impl EthernetDeviceCapabilities
{
	const ImmediateStart: u8 = 0;
	
	/// Receive threshold.
	#[inline(always)]
	pub fn receive_threshold(&self) -> rte_eth_thresh
	{
		self.0.default_rxconf.rx_thresh
	}
	
	/// Receive free threshold.
	#[inline(always)]
	pub fn receive_free_threshold(&self) -> u16
	{
		self.0.default_rxconf.rx_free_thresh
	}
	
	/// Receive hardware offloading flags for what the ethernet device supports for a receive queue.
	#[inline(always)]
	pub fn receive_queue_hardware_offloading_flags(&self) -> ReceiveHardwareOffloadingFlags
	{
		ReceiveHardwareOffloadingFlags::from_bits_truncate(self.0.rx_queue_offload_capa)
	}
	
	/// Transmit threshold.
	#[inline(always)]
	pub fn transmit_threshold(&self) -> rte_eth_thresh
	{
		self.0.default_txconf.tx_thresh
	}
	
	/// Transmit free threshold.
	#[inline(always)]
	pub fn transmit_free_threshold(&self) -> u16
	{
		self.0.default_txconf.tx_free_thresh
	}
	
	/// Transmit 'RS' threshold.
	#[inline(always)]
	pub fn transmit_rs_threshold(&self) -> u16
	{
		self.0.default_txconf.tx_rs_thresh
	}
	
	/// Transmit hardware offloading flags for what the ethernet device supports for a transmit queue.
	#[inline(always)]
	pub fn transmit_queue_hardware_offloading_flags(&self) -> TransmitHardwareOffloadingFlags
	{
		TransmitHardwareOffloadingFlags::from_bits_truncate(self.0.tx_queue_offload_capa)
	}
	
	/// Receive side scaling hash key size.
	///
	/// Should be either 40 or 52.
	#[inline(always)]
	pub fn hash_key_size(&self) -> ReceiveSideScalingHashKeySize
	{
		use self::ReceiveSideScalingHashKeySize::*;
		
		match self.0.hash_key_size
		{
			// Some drivers, such as Mellanox's ?still report zero when they mean 40.
			0 => Forty,
			
			40 => Forty,
			52 => FiftyTwo,
			
			hash_key_size @ _ => panic!("Unsupported hash_key_size '{}'", hash_key_size)
		}
	}
	
	/// Last receive queue.
	///
	/// Returns `None` if `first_queue` exceeds those possible.
	#[inline(always)]
	pub fn last_receive_queue(&self, first_receive_queue: ReceiveQueueIdentifier, any_number_of_receive_queues: usize) -> Option<ReceiveQueueIdentifier>
	{
		if any_number_of_receive_queues == 0
		{
			return Some(first_receive_queue)
		}
		
		let first_receive_queue: u16 = first_receive_queue.into();
		let limit_number_of_receive_queues: u16 = self.limit_number_of_receive_queues(any_number_of_receive_queues).into();
		if first_receive_queue >= limit_number_of_receive_queues
		{
			return None
		}
		
		let last_receive_queue = min(first_receive_queue.saturating_add(limit_number_of_receive_queues), limit_number_of_receive_queues - 1);
		Some(ReceiveQueueIdentifier(last_receive_queue))
	}
	
	/// Limits the number of receive queues to the device supported maximum.
	#[inline(always)]
	pub fn limit_number_of_receive_queues(&self, any_number_of_receive_queues: usize) -> ReceiveNumberOfQueues
	{
		ReceiveNumberOfQueues(min(self.0.max_rx_queues as usize, any_number_of_receive_queues) as u16)
	}
	
	/// Limits the number of transmit queues to the device supported maximum.
	#[inline(always)]
	pub fn limit_number_of_transmit_queues(&self, any_number_of_transmit_queues: usize) -> TransmitNumberOfQueues
	{
		TransmitNumberOfQueues(min(self.0.max_rx_queues as usize, any_number_of_transmit_queues) as u16)
	}
	
	/// Internally, the device must support one of the supported DPDK sizes:-
	///
	/// * `ETH_RSS_RETA_SIZE_64`
	/// * `ETH_RSS_RETA_SIZE_128`
	/// * `ETH_RSS_RETA_SIZE_256`
	/// * `ETH_RSS_RETA_SIZE_512`
	///
	/// If it is not, this code will panic.
	#[inline(always)]
	pub fn redirection_table_number_of_entries(&self) -> RedirectionTableNumberOfEntries
	{
		use self::RedirectionTableNumberOfEntries::*;
		
		match self.0.reta_size
		{
			ETH_RSS_RETA_SIZE_64 => Entries64,
			ETH_RSS_RETA_SIZE_128 => Entries128,
			ETH_RSS_RETA_SIZE_256 => Entries256,
			ETH_RSS_RETA_SIZE_512 => Entries512,
			
			reta_size @ _ => panic!("Unsupported reta_size '{}'", reta_size)
		}
	}
}
