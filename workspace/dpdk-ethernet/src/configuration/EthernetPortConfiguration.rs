// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ethernet port configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct EthernetPortConfiguration
{
	/// Receive side scaling configuration.
	#[serde(default = "EthernetPortConfiguration::isolate_flow_rules_default")]
	pub isolate_flow_rules: bool,
	
	/// Set a specifc media access control address.
	///
	/// Generally speaking, this is a good idea.
	#[serde(default)]
	pub media_access_control_address: Option<MediaAccessControlAddress>,
	
	/// Override packet buffer pools; merged with the defaults to provide references to packet buffer pools.
	#[serde(default)]
	pub override_packet_buffer_pools: HashMap<NumaNode, PacketBufferPoolReference>,
	
	/// Receive queue configurations.
	pub receive_queue_configurations: Box<[ReceiveQueueConfiguration]>,
	
	/// Transmit queue configurations.
	pub transmit_queue_configurations: Box<[TransmitQueueConfiguration]>,
	
	/// Receive side scaling configuration.
	#[serde(default)]
	pub receive_side_scaling_configuration: Option<ReceiveSideScalingConfiguration>,
}

impl EthernetPortConfiguration
{
	/// Configure.
	#[cold]
	pub fn configure(&self, ethernet_port_identifier: EthernetPortIdentifier, packet_buffer_pools_by_numa_node: &[PacketBufferPoolReference; NumaNode::Maximum], packet_buffer_pools: &HashMap<PacketBufferPoolReference, PacketBufferPool>) -> (EthernetDeviceCapabilities, Box<[ReceiveBurst]>, Box<[TransmitBurst]>)
	{
		let ethernet_device_capabilities = ethernet_port_identifier.ethernet_device_capabilities();
		
		ethernet_device_capabilities.validate_not_too_many_receive_queues(self.receive_queue_configurations.len());
		ethernet_device_capabilities.validate_not_too_many_transmit_queues(self.transmit_queue_configurations.len());
		
		if let Some(media_access_control_address) = self.media_access_control_address
		{
			ethernet_port_identifier.configure_default_media_access_control_address(media_access_control_address);
		}
		
		if self.isolate_flow_rules
		{
			ethernet_port_identifier.configure_flow_isolation().unwrap();
		}
		
		ethernet_port_identifier.configure_ethernet_device(&ethernet_device_capabilities, &self.receive_queue_configurations[..], &self.transmit_queue_configurations[..], self.receive_side_scaling_configuration.as_ref());
		
		let transmit_bursts = self.configure_transmit_queues(ethernet_port_identifier, &ethernet_device_capabilities);
		
		let receive_bursts = self.configure_receive_queues(ethernet_port_identifier, &ethernet_device_capabilities, packet_buffer_pools_by_numa_node, packet_buffer_pools);
		
		(ethernet_device_capabilities, receive_bursts, transmit_bursts)
	}
	
	#[inline(always)]
	fn configure_transmit_queues(&self, ethernet_port_identifier: EthernetPortIdentifier, ethernet_device_capabilities: &EthernetDeviceCapabilities) -> Box<[TransmitBurst]>
	{
		let default_ethernet_device_transmit_queue_capabilities = ethernet_device_capabilities.ethernet_device_transmit_queue_capabilities();
		let mut queue_identifier = TransmitQueueIdentifier::Zero;
		let mut transmit_bursts = Vec::with_capacity(self.transmit_queue_configurations.len());
		for transmit_queue_configuration in self.transmit_queue_configurations.iter()
		{
			transmit_bursts.push(transmit_queue_configuration.configure(ethernet_port_identifier, queue_identifier, default_ethernet_device_transmit_queue_capabilities, ethernet_device_capabilities.transmit_queue_ring_size_constraints()));
			queue_identifier += 1u16;
		}
		transmit_bursts.into_boxed_slice()
	}
	
	#[inline(always)]
	fn configure_receive_queues(&self, ethernet_port_identifier: EthernetPortIdentifier, ethernet_device_capabilities: &EthernetDeviceCapabilities, packet_buffer_pools_by_numa_node: &[PacketBufferPoolReference; NumaNode::Maximum], packet_buffer_pools: &HashMap<PacketBufferPoolReference, PacketBufferPool>) -> Box<[ReceiveBurst]>
	{
		let packet_buffer_pool_references = self.packet_buffer_pool_references(packet_buffer_pools_by_numa_node);
		
		let default_ethernet_device_receive_queue_capabilities = ethernet_device_capabilities.ethernet_device_receive_queue_capabilities();
		let mut queue_identifier = ReceiveQueueIdentifier::Zero;
		let mut receive_bursts = Vec::with_capacity(self.receive_queue_configurations.len());
		for receive_queue_configuration in self.receive_queue_configurations.iter()
		{
			receive_bursts.push(receive_queue_configuration.configure(ethernet_port_identifier, queue_identifier, default_ethernet_device_receive_queue_capabilities, ethernet_device_capabilities.receive_queue_ring_size_constraints(), &packet_buffer_pool_references, packet_buffer_pools));
			queue_identifier += 1u16;
		}
		receive_bursts.into_boxed_slice()
	}
	
	#[inline(always)]
	fn packet_buffer_pool_references(&self, packet_buffer_pools_by_numa_node: &[PacketBufferPoolReference; NumaNode::Maximum]) -> HashMap<NumaNode, PacketBufferPoolReference>
	{
		let mut packet_buffer_pools = HashMap::with_capacity(NumaNode::Maximum);
		for numa_node_index in 0u16 .. (NumaNode::Maximum as u16)
		{
			packet_buffer_pools.insert(NumaNode::from_u16(numa_node_index), *(unsafe { packet_buffer_pools_by_numa_node.get_unchecked(numa_node_index as usize) }));
		}
		
		for (numa_node, packet_buffer_pool_reference) in self.override_packet_buffer_pools.iter()
		{
			packet_buffer_pools.insert(*numa_node, *packet_buffer_pool_reference);
		}
		
		packet_buffer_pools
	}
	
	#[inline(always)]
	fn isolate_flow_rules_default() -> bool
	{
		true
	}
}
