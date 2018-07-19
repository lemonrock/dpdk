// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PacketProcessingForQinQVirtualLanConfiguration
{
	/// Outer QinQ Virtual LAN permitted classes of service.
	pub outer_packet_processing: PacketProcessingConfiguration,
	
	/// Inner packet processing configuration.
	pub inner_packet_processing: PacketProcessingConfiguration,
}

impl PacketProcessingForQinQVirtualLanConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<PPDO: PacketProcessingDropObserver>(mut self, our_valid_unicast_ethernet_address: MediaAccessControlAddress, numa_node_choice: NumaNodeChoice, dropped_packet_reporting: &Rc<PPDO>) -> PacketProcessingForQinQVirtualLan<PPDO>
	{
		PacketProcessingForQinQVirtualLan
		{
			outer_packet_processing: self.outer_packet_processing.configure(our_valid_unicast_ethernet_address, numa_node_choice, dropped_packet_reporting),
			inner_packet_processing: self.inner_packet_processing.configure(our_valid_unicast_ethernet_address, numa_node_choice, dropped_packet_reporting),
		}
	}
}
