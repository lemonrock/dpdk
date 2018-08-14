// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ethernet device information.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct EthernetDeviceCapabilities
{
	driver_name: &'static str,
	maximum_queue_pairs: u16,
	maximum_receive_packet_length: u16,
	receive_side_scaling_offload_flow: ReceiveSideScalingOffloadFlow,
	receive_device_hardware_offloading_flags: ReceiveHardwareOffloadingFlags,
	receive_queue_hardware_offloading_flags: ReceiveHardwareOffloadingFlags,
	transmit_device_hardware_offloading_flags: TransmitHardwareOffloadingFlags,
	transmit_queue_hardware_offloading_flags: TransmitHardwareOffloadingFlags,
	receive_side_scaling_is_unavailable: bool,
	receive_side_scaling_hash_key_size: Option<ReceiveSideScalingHashKeySize>,
	redirection_table_number_of_entries: Option<RedirectionTableNumberOfEntries>,
	extended_statistics_names: Vec<&'static str>,
	dpdk_information: rte_eth_dev_info,
}

impl EthernetDeviceCapabilities
{
	const ImmediateStart: u8 = 0;
	
	#[inline(always)]
	pub(crate) fn from(mut dpdk_information: rte_eth_dev_info, extended_statistics_names: Vec<&'static str>) -> Self
	{
		let driver_name = unsafe { CStr::from_ptr(dpdk_information.driver_name) }.to_str().unwrap();
		
		let receive_side_scaling_is_unavailable = dpdk_information.flow_type_rss_offloads == 0 || dpdk_information.reta_size == 0;
		
		Self
		{
			driver_name,
			maximum_queue_pairs:
			{
				let maximum_transmit_queue = dpdk_information.max_tx_queues;
				debug_assert_ne!(maximum_transmit_queue, 0, "maximum transmit queues is zero");
				
				let possibly_buggy_max_rx_queues = dpdk_information.max_rx_queues;
				debug_assert_ne!(possibly_buggy_max_rx_queues, 0, "maximum receive queues is zero");
				
				let maximum_receieve_queues = match driver_name
				{
					"rte_i40e_pmd" => min(possibly_buggy_max_rx_queues, 64),
					"rte_i40evf_pmd" => min(possibly_buggy_max_rx_queues, 16),
					"rte_ixgbe_pmd" => min(possibly_buggy_max_rx_queues, 16),
					"rte_ixgbevf_pmd" => min(possibly_buggy_max_rx_queues, 4),
					
					_ => possibly_buggy_max_rx_queues,
				};
				dpdk_information.max_rx_queues = maximum_receieve_queues;
				
				min(maximum_receieve_queues, maximum_transmit_queue)
			},
			maximum_receive_packet_length:
			{
				// Some drivers use nonsense values that exceed super jumbo frame sizes for `max_rx_pktlen`.
				let possibly_buggy_max_rx_pktlen = dpdk_information.max_rx_pktlen;
				let into: u16 = EthernetFrameLength::MaximumIncludingCyclicRedundancyCheckWithJumboFrames.into();
				let maximum_receive_packet_length = min(possibly_buggy_max_rx_pktlen, into as u32);
				dpdk_information.max_rx_pktlen = maximum_receive_packet_length;
				maximum_receive_packet_length as u16
			},
			receive_side_scaling_offload_flow: ReceiveSideScalingOffloadFlow::from_bits_truncate(dpdk_information.flow_type_rss_offloads),
			receive_device_hardware_offloading_flags: ReceiveHardwareOffloadingFlags::from_bits_truncate(dpdk_information.rx_offload_capa),
			receive_queue_hardware_offloading_flags: ReceiveHardwareOffloadingFlags::from_bits_truncate(dpdk_information.rx_queue_offload_capa),
			transmit_device_hardware_offloading_flags: TransmitHardwareOffloadingFlags::from_bits_truncate(dpdk_information.tx_offload_capa),
			transmit_queue_hardware_offloading_flags: TransmitHardwareOffloadingFlags::from_bits_truncate(dpdk_information.tx_queue_offload_capa),
			receive_side_scaling_is_unavailable,
			receive_side_scaling_hash_key_size:
			{
				if receive_side_scaling_is_unavailable
				{
					None
				}
				else
				{
					use self::ReceiveSideScalingHashKeySize::*;
					match dpdk_information.hash_key_size
					{
						// Some drivers, such as Mellanox's ?still report zero when they mean 40.
						0 => Some(Forty),
						
						40 => Some(Forty),
						52 => Some(FiftyTwo),
						
						hash_key_size @ _ => panic!("Unsupported hash_key_size '{}'", hash_key_size)
					}
				}
			},
			redirection_table_number_of_entries:
			{
				if receive_side_scaling_is_unavailable
				{
					None
				}
				else
				{
					use self::RedirectionTableNumberOfEntries::*;
					match dpdk_information.reta_size
					{
						ETH_RSS_RETA_SIZE_64 => Some(Entries64),
						ETH_RSS_RETA_SIZE_128 => Some(Entries128),
						ETH_RSS_RETA_SIZE_256 => Some(Entries256),
						ETH_RSS_RETA_SIZE_512 => Some(Entries512),
						
						reta_size @ _ => panic!("Unsupported reta_size '{}'", reta_size)
					}
				}
			},
			extended_statistics_names,
			dpdk_information,
		}
	}
	
	/// An ExtendedStatisticsIterator is slightly expensive to construct, so it should be re-used.
	pub fn extended_statistics_iterator<'a>(&'a self) -> ExtendedStatisticsIterator<'a>
	{
		ExtendedStatisticsIterator::new_unchecked(&self.extended_statistics_names)
	}
	
	/// Maximum receive packet length.
	#[inline(always)]
	pub fn maximum_receive_packet_length(&self) -> EthernetFrameLength
	{
		EthernetFrameLength::try_from_with_jumbo_frames(self.maximum_receive_packet_length)
	}
	
	/// Receive side scaling supported offload flows.
	#[inline(always)]
	pub fn receive_side_scaling_offload_flow(&self) -> ReceiveSideScalingOffloadFlow
	{
		self.receive_side_scaling_offload_flow
	}
	
	/// Receive threshold.
	#[inline(always)]
	pub fn receive_threshold(&self) -> rte_eth_thresh
	{
		self.dpdk_information.default_rxconf.rx_thresh
	}
	
	/// Receive free threshold.
	#[inline(always)]
	pub fn receive_free_threshold(&self) -> u16
	{
		self.dpdk_information.default_rxconf.rx_free_thresh
	}
	
	/// Receive hardware offloading flags for what the ethernet device supports generally.
	pub fn receive_device_hardware_offloading_flags(&self) -> ReceiveHardwareOffloadingFlags
	{
		self.receive_device_hardware_offloading_flags
	}
	
	/// Receive hardware offloading flags for what the ethernet device supports for a receive queue.
	#[inline(always)]
	pub fn receive_queue_hardware_offloading_flags(&self) -> ReceiveHardwareOffloadingFlags
	{
		self.receive_queue_hardware_offloading_flags
	}
	
	#[inline(always)]
	pub(crate) fn receive_queue_ring_size(&self) -> ReceiveQueueRingSize
	{
		ReceiveQueueRingSize(self.dpdk_information.rx_desc_lim.nb_max)
	}
	
	/// Limits the number of receive queues to the device supported maximum queue pairs.
	#[inline(always)]
	pub fn limit_number_of_receive_queues(&self, any_number_of_receive_queues: usize) -> ReceiveNumberOfQueues
	{
		ReceiveNumberOfQueues(min(self.maximum_queue_pairs as usize, any_number_of_receive_queues) as u16)
	}
	
	/// Receive burst maximum packets.
	#[inline(always)]
	pub fn receive_burst_maximum_packets(&self) -> usize
	{
		self.dpdk_information.default_rxportconf.burst_size as usize
	}
	
	/// Transmit threshold.
	#[inline(always)]
	pub fn transmit_threshold(&self) -> rte_eth_thresh
	{
		self.dpdk_information.default_txconf.tx_thresh
	}
	
	/// Transmit free threshold.
	#[inline(always)]
	pub fn transmit_free_threshold(&self) -> u16
	{
		self.dpdk_information.default_txconf.tx_free_thresh
	}
	
	/// Transmit 'RS' threshold.
	#[inline(always)]
	pub fn transmit_rs_threshold(&self) -> u16
	{
		self.dpdk_information.default_txconf.tx_rs_thresh
	}
	
	/// Transmit hardware offloading flags for what the ethernet device supports generally.
	pub fn transmit_device_hardware_offloading_flags(&self) -> TransmitHardwareOffloadingFlags
	{
		self.transmit_device_hardware_offloading_flags
	}
	
	/// Transmit hardware offloading flags for what the ethernet device supports for a transmit queue.
	#[inline(always)]
	pub fn transmit_queue_hardware_offloading_flags(&self) -> TransmitHardwareOffloadingFlags
	{
		self.transmit_queue_hardware_offloading_flags
	}
	
	#[inline(always)]
	pub(crate) fn transmit_queue_ring_size(&self) -> TransmitQueueRingSize
	{
		TransmitQueueRingSize(self.dpdk_information.tx_desc_lim.nb_max)
	}
	
	/// Limits the number of transmit queues to the device supported maximum queue pairs.
	#[inline(always)]
	pub fn limit_number_of_transmit_queues(&self, any_number_of_transmit_queues: usize) -> TransmitNumberOfQueues
	{
		TransmitNumberOfQueues(min(self.maximum_queue_pairs as usize, any_number_of_transmit_queues) as u16)
	}
	
	/// Transmit burst maximum packets.
	#[inline(always)]
	pub fn transmit_burst_maximum_packets(&self) -> usize
	{
		self.dpdk_information.default_txportconf.burst_size as usize
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
	
	/// Is receive side scaling unavailable?
	#[inline(always)]
	pub fn receive_side_scaling_is_unavailable(&self) -> bool
	{
		self.receive_side_scaling_is_unavailable
	}
	
	/// Receive side scaling hash key size.
	///
	/// Should be either 40 or 52.
	#[inline(always)]
	pub fn receive_side_scaling_hash_key_size(&self) -> Option<ReceiveSideScalingHashKeySize>
	{
		self.receive_side_scaling_hash_key_size
	}
	
	/// Internally, the device must support one of the supported DPDK sizes:-
	///
	/// * `ETH_RSS_RETA_SIZE_64`
	/// * `ETH_RSS_RETA_SIZE_128`
	/// * `ETH_RSS_RETA_SIZE_256`
	/// * `ETH_RSS_RETA_SIZE_512`
	#[inline(always)]
	pub fn redirection_table_number_of_entries(&self) -> Option<RedirectionTableNumberOfEntries>
	{
		self.redirection_table_number_of_entries
	}
}
