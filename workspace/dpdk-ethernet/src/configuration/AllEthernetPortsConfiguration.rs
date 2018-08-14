// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// All ethernet ports configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct AllEthernetPortsConfiguration
{
	/// Packet buffer pool definitions.
	pub packet_buffer_pool_definitions: HashMap<PacketBufferPoolReference, PacketBufferPoolConfiguration>,
	
	/// Packet buffer pools by NUMA node.
	#[serde(default)]
	pub packet_buffer_pools_by_numa_node: [PacketBufferPoolReference; NumaNode::Maximum],
	
	/// BROKEN - ethernet port identifier is unknowable.
	pub ethernet_ports: HashMap<EthernetPortIdentifier, EthernetPortConfiguration>,
}

impl AllEthernetPortsConfiguration
{
	/// Configure.
	pub fn configure(&self) -> (HashSet<PacketBufferPool>, HashMap<EthernetPortIdentifier, (EthernetDeviceCapabilities, Box<[ReceiveBurst]>, Box<[TransmitBurst]>)>)
	{
		let packet_buffer_pools_to_not_drop = self.configure_packet_buffer_pools();
		
		let mut ethernet_ports = HashMap::with_capacity(self.ethernet_ports.len());
		
		for (ethernet_port_identifier, ethernet_port_configuration) in self.ethernet_ports
		{
			let ethernet_port_identifier = *ethernet_port_identifier;
			ethernet_ports.push(ethernet_port_identifier, ethernet_port_configuration.configure((ethernet_port_identifier, &self.packet_buffer_pools_by_numa_node));
		}
		
		(packet_buffer_pools_to_not_drop, ethernet_ports)
	}
	
	fn configure_packet_buffer_pools(&self) -> HashSet<PacketBufferPool>
	{
		for packet_buffer_pool_reference in self.packet_buffer_pools_by_numa_node.iter()
		{
			assert!(self.packet_buffer_pool_definitions.contains_key(packet_buffer_pool_reference), "packet_buffer_pools_by_numa_node '{:?}' is missing in packet_buffer_pool_definitions", packet_buffer_pool_reference);
		}
		
		let mut packet_buffer_pools_to_not_drop = HashSet::with_capacity(self.packet_buffer_pool_definitions.len());
		
		for (packet_buffer_bool_reference, packet_buffer_pool_configuration) in self.packet_buffer_pool_definitions.iter()
		{
			packet_buffer_pools_to_not_drop.push(packet_buffer_pool_configuration.configure(packet_buffer_pool_reference).unwrap())
		}
		
		packet_buffer_pools_to_not_drop
	}
}
