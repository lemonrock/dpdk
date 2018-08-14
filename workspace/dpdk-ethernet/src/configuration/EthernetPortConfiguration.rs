// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ethernet port configuration.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EthernetPortConfiguration
{
	//pub default_packet_buffer_pools: HashMap<NumaNode, PacketBufferPool>,
	
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
	#[inline(always)]
	pub fn configure(&self, ethernet_port_identifier: EthernetPortIdentifier, ethernet_device_capabilities: &EthernetDeviceCapabilities) -> (Box<[ReceiveBurst]>, Box<[TransmitBurst]>)
	{
		ethernet_port_identifier.configure_ethernet_device(ethernet_device_capabilities, &self.receive_queue_configurations[..], &self.transmit_queue_configurations[..], self.receive_side_scaling_configuration.as_ref());
		
		let default_ethernet_device_transmit_queue_capabilities = ethernet_device_capabilities.ethernet_device_transmit_queue_capabilities();
		let mut queue_identifier = TransmitQueueIdentifier::Zero;
		let mut transmit_bursts = Vec::with_capacity(self.transmit_queue_configurations.len());
		for transmit_queue_configuration in self.transmit_queue_configurations.iter()
		{
			transmit_bursts.push(transmit_queue_configuration.configure(ethernet_port_identifier, queue_identifier, default_ethernet_device_transmit_queue_capabilities));
			queue_identifier += 1u16;
		}
		
		let default_ethernet_device_receive_queue_capabilities = ethernet_device_capabilities.ethernet_device_receive_queue_capabilities();
		let mut queue_identifier = ReceiveQueueIdentifier::Zero;
		let mut receive_bursts = Vec::with_capacity(self.receive_queue_configurations.len());
		for receive_queue_configuration in self.receive_queue_configurations.iter()
		{
			receive_bursts.push(receive_queue_configuration.configure(ethernet_port_identifier, queue_identifier, default_ethernet_device_receive_queue_capabilities));
			queue_identifier += 1u16;
		}
		
		(receive_bursts.into_boxed_slice(), transmit_bursts.into_boxed_slice())
	}
}
