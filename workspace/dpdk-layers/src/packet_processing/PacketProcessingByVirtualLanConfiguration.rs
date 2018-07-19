// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PacketProcessingByVirtualLanConfiguration
{
	/// Outer QinQ Virtual LAN.
	pub outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), PacketProcessingForQinQVirtualLanConfiguration>,
	
	/// Inner 802.1Q Virtual LAN.
	pub inner: HashMap<VirtualLanIdentifier, PacketProcessingConfiguration>,
	
	/// No virtual LANs.
	pub none: PacketProcessingConfiguration,
}

impl PacketProcessingByVirtualLanConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<PPDO: PacketProcessingDropOberserver>(mut self, our_valid_unicast_ethernet_address: MediaAccessControlAddress, numa_node_choice: NumaNodeChoice, dropped_packet_reporting: &Rc<PPDO>) -> PacketProcessingByVirtualLan<PPDO>
	{
		PacketProcessingByVirtualLan
		{
			outer: self.outer.drain().map(|(key, value)| (key, value.configure(our_valid_unicast_ethernet_address, numa_node_choice, dropped_packet_reporting))).collect(),
			
			inner: self.outer.drain().map(|(key, value)| (key, value.configure(our_valid_unicast_ethernet_address, numa_node_choice, dropped_packet_reporting))).collect(),
			
			none: self.none.configure(self.our_valid_unicast_ethernet_address, numa_node_choice, dropped_packet_reporting),
		}
	}
}
