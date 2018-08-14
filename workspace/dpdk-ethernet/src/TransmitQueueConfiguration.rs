// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Transmit queue configuration.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct TransmitQueueConfiguration
{
	/// Override the ethernet device transmit queue capabilities.
	pub overrride_ethernet_device_transmit_queue_capabilities: Option<EthernetDeviceTransmitQueueCapabilities>,
	
	/// Specify the transmit hardware offloading flags.
	pub hardware_offloading_flags: TransmitHardwareOffloadingFlags,
	
	/// Queue ring ring size.
	pub queue_ring_size: TransmitQueueRingSize,
	
	/// Override the queue ring's NUMA node from that used for the ethernet port.
	pub queue_ring_numa_node: Option<NumaNode>,
	
	/// Counter index for simple statistics (shared across one or more transmit and receive queues).
	pub queue_simple_statistics_counter_index: QueueSimpleStatisticCounterIndex,
}

impl TransmitQueueConfiguration
{
	#[inline(always)]
	pub(crate) fn configure(&self, ethernet_port_identifier: EthernetPortIdentifier, queue_identifier: TransmitQueueIdentifier, default_ethernet_device_transmit_queue_capabilities: &EthernetDeviceTransmitQueueCapabilities) -> TransmitBurst
	{
		let ethernet_device_transmit_queue_capabilities = if let Some(ref ethernet_device_transmit_queue_capabilities) = self.overrride_ethernet_device_transmit_queue_capabilities
		{
			ethernet_device_transmit_queue_capabilities
		}
		else
		{
			default_ethernet_device_transmit_queue_capabilities
		};
		
		let queue_ring_numa_node = if let Some(queue_ring_numa_node) = self.queue_ring_numa_node
		{
			queue_ring_numa_node
		}
		else
		{
			ethernet_port_identifier.numa_node_choice().unwrap_or_default()
		};
		
		let queue_configuration = rte_eth_txconf
		{
			tx_thresh: ethernet_device_transmit_queue_capabilities.transmit_threshold().into(),
			tx_rs_thresh: ethernet_device_transmit_queue_capabilities.transmit_intel_specific_rs_bit_threshold(),
			tx_free_thresh: ethernet_device_transmit_queue_capabilities.transmit_free_threshold(),
			txq_flags: ETH_TXQ_FLAGS_IGNORE,
			tx_deferred_start: EthernetDeviceCapabilities::ImmediateStart,
			offloads: (ethernet_device_transmit_queue_capabilities.transmit_queue_hardware_offloading_flags() & self.hardware_offloading_flags).bits(),
		};
		
		let result = unsafe { rte_eth_tx_queue_setup(ethernet_port_identifier.into(), queue_identifier.into(), self.queue_ring_size.into(), queue_ring_numa_node.into(), &queue_configuration) };
		
		if likely!(result == 0)
		{
			let into: u8 = self.queue_simple_statistics_counter_index.into();
			let result = unsafe { rte_eth_dev_set_tx_queue_stats_mapping(ethernet_port_identifier.into(), queue_identifier.into(), into) };
			if likely!(result == 0)
			{
				return TransmitBurst::new(ethernet_port_identifier, ethernet_device_transmit_queue_capabilities, queue_identifier)
			}
			else
			{
				panic!("rte_eth_dev_set_tx_queue_stats_mapping for ethernet port '{}' for queue '{}' failed with '{}'", ethernet_port_identifier, queue_identifier, result)
			}
		}
		
		match result
		{
			// NOTE: This is not listed in the documentation but it seems likely to occur.
			NegativeE::ENODEV => panic!("This ethernet port '{}' for queue '{}' is not a device", ethernet_port_identifier, queue_identifier),
			
			// NOTE: This is not listed in the documentation but it seems likely to occur.
			NegativeE::EIO => panic!("This ethernet port '{}' for queue '{}' is removed", ethernet_port_identifier, queue_identifier),
			
			NegativeE::ENOMEM => panic!("rte_eth_tx_queue_setup: unable to allocate the transmit ring descriptors for ethernet port '{}' for queue '{}'", ethernet_port_identifier, queue_identifier),
			
			_ => panic!("rte_eth_rx_queue_setup returned an unknown error '{}' for ethernet port '{}' for queue '{}'", result, ethernet_port_identifier, queue_identifier)
		}
	}
}
