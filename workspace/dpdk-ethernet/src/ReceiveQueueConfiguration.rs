// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive queue configuration.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ReceiveQueueConfiguration
{
	/// Override the ethernet device receive queue capabilities.
	pub overrride_ethernet_device_receive_queue_capabilities: Option<EthernetDeviceReceiveQueueCapabilities>,
	
	/// Specify the receive hardware offloading flags.
	pub hardware_offloading_flags: ReceiveHardwareOffloadingFlags,
	
	/// Queue ring ring size.
	pub queue_ring_size: TransmitQueueRingSize,
	
	/// Override the queue ring's NUMA node from that used for the ethernet port.
	pub queue_ring_numa_node: Option<NumaNode>,
	
	/// Counter index for simple statistics (shared across one or more transmit and receive queues).
	pub queue_simple_statistics_counter_index: QueueSimpleStatisticCounterIndex,
}

impl ReceiveQueueConfiguration
{
	/// `queue_packet_buffer_pool` should ideally be on the same NUMA node as that used for the ethernet port.
	#[inline(always)]
	pub(crate) fn configure(&self, ethernet_port_identifier: EthernetPortIdentifier, queue_identifier: ReceiveQueueIdentifier, default_ethernet_device_receive_queue_capabilities: &EthernetDeviceReceiveQueueCapabilities, queue_packet_buffer_pool: NonNull<rte_mempool>) -> ReceiveBurst
	{
		let ethernet_device_receive_queue_capabilities = self.overrride_ethernet_device_receive_queue_capabilities.as_ref().unwrap_or(default_ethernet_device_receive_queue_capabilities);
		let queue_ring_numa_node = self.queue_ring_numa_node.unwrap_or_else(|| ethernet_port_identifier.numa_node_choice().unwrap_or_default());
		
		let queue_configuration =
		{
			const DropPacketsIfNoReceiveDescriptorsAreAvailable: u8 = 1;
			
			rte_eth_rxconf
			{
				rx_thresh: ethernet_device_receive_queue_capabilities.receive_threshold().into(),
				rx_free_thresh: ethernet_device_receive_queue_capabilities.receive_free_threshold(),
				rx_drop_en: DropPacketsIfNoReceiveDescriptorsAreAvailable,
				rx_deferred_start: EthernetDeviceCapabilities::ImmediateStart,
				offloads: (ethernet_device_receive_queue_capabilities.receive_queue_hardware_offloading_flags() & self.hardware_offloading_flags).bits(),
			}
		};
		
		let result = unsafe { rte_eth_rx_queue_setup(ethernet_port_identifier.into(), queue_identifier.into(), self.queue_ring_size.into(), queue_ring_numa_node.into(), &queue_configuration, queue_packet_buffer_pool.as_ptr()) };
		
		if likely!(result == 0)
		{
			let into: u8 = self.queue_simple_statistics_counter_index.into();
			let result = unsafe { rte_eth_dev_set_rx_queue_stats_mapping(ethernet_port_identifier.into(), queue_identifier.into(), into) };
			
			if likely!(result == 0)
			{
				return ReceiveBurst::new(ethernet_port_identifier, ethernet_device_receive_queue_capabilities, queue_identifier)
			}
			else
			{
				panic!("rte_eth_dev_set_rx_queue_stats_mapping for ethernet port '{}' for queue '{}' failed with '{}'", ethernet_port_identifier, queue_identifier, result)
			}
		}
		
		match result
		{
			// NOTE: This is not listed in the documentation but it seems likely to occur.
			NegativeE::ENODEV => panic!("This ethernet port '{}' for queue '{}' is not a device", ethernet_port_identifier, queue_identifier),
			
			NegativeE::EIO => panic!("This ethernet port '{}' for queue '{}' is removed", ethernet_port_identifier, queue_identifier),
			NegativeE::EINVAL => panic!("rte_eth_rx_queue_setup: the size of network buffers which can be allocated from the memory pool does not fit the various buffer sizes allowed by the device controller for ethernet port '{}' for queue '{}'", ethernet_port_identifier, queue_identifier),
			NegativeE::ENOMEM => panic!("rte_eth_rx_queue_setup: unable to allocate the receive ring descriptors or to allocate network packet buffers from the queue_packet_buffer_pool when initializing receive descriptors for ethernet port '{}' for queue '{}'", ethernet_port_identifier, queue_identifier),
			
			_ => panic!("rte_eth_rx_queue_setup returned an unknown error '{}' for ethernet port '{}' for queue '{}'", result, ethernet_port_identifier, queue_identifier)
		}
	}
}
