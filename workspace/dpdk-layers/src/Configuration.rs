// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Configuration for packet processing.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Configuration
{
	/// Our unicast ethernet addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	pub our_valid_unicast_ethernet_address: MediaAccessControlAddress,
	
	/// Root of all packet processing.
	pub packet_processing_by_virtual_lan_configuration: PacketProcessingByVirtualLanConfiguration,
}

impl Configuration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<PPDO: PacketProcessingDropOberserver>(mut self, numa_node_choice: NumaNodeChoice, dropped_packet_reporting: &Rc<PPDO>) -> PacketProcessingByVirtualLan<PPDO>
	{
		self.packet_processing_by_virtual_lan_configuration.configure(self.our_valid_unicast_ethernet_address, numa_node_choice, dropped_packet_reporting)
	}
}
