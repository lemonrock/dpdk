// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct PacketProcessingConfiguration
{
	/// Inner 802.1Q Virtual LAN honour drop eligible.
	#[serde(default = "PacketProcessingConfiguration::inner_honour_drop_eligible_indicator_default")] inner_honour_drop_eligible_indicator: bool,
	
	/// Inner 802.1Q Virtual LAN permitted classes of service.
	#[serde(default)] pub inner_permitted_classes_of_service: PermittedClassesOfService,
	
	/// Blacklist or whitelist of ethernet addresses.
	#[serde(default)] pub source_ethernet_address_blacklist_or_whitelist: MediaAccessControlAddressList,
	
	/// Our unicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	#[serde(default)] pub our_valid_internet_protocol_version_4_host_addresses: HashSet<InternetProtocolVersion4HostAddress>,
	
	/// Packet reassembly configuration for fragmented packets for Internet Protocol (IP) version 4.
	#[serde(default)] pub internet_protocol_version_4_packet_reassembly_table_configuration: InternetProtocolPacketReassemblyTableConfiguration,
	
	/// Packet reassembly configuration for fragmented packets for Internet Protocol (IP) version 6.
	#[serde(default)] pub internet_protocol_version_6_packet_reassembly_table_configuration: InternetProtocolPacketReassemblyTableConfiguration,
}

impl PacketProcessingConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<PPDO: PacketProcessingDropObserver>(mut self, our_valid_unicast_ethernet_address: MediaAccessControlAddress, numa_node_choice: NumaNodeChoice, dropped_packet_reporting: &Rc<PPDO>) -> PacketProcessing<PPDO>
	{
		PacketProcessing
		{
			inner_honour_drop_eligible_indicator: self.inner_honour_drop_eligible_indicator,
			inner_permitted_classes_of_service: self.inner_permitted_classes_of_service,
			our_valid_unicast_ethernet_address,
			source_ethernet_address_blacklist_or_whitelist: self.source_ethernet_address_blacklist_or_whitelist,
			our_valid_internet_protocol_version_4_host_addresses: self.our_valid_internet_protocol_version_4_host_addresses,
			internet_protocol_version_4_packet_reassembly_table: self.internet_protocol_version_4_packet_reassembly_table_configuration.create_table(numa_node_choice).expect("out of memory"),
			internet_protocol_version_6_packet_reassembly_table: self.internet_protocol_version_6_packet_reassembly_table_configuration.create_table(numa_node_choice).expect("out of memory"),
			dropped_packet_reporting: dropped_packet_reporting.clone(),
		}
	}
	
	#[inline(always)]
	fn inner_honour_drop_eligible_indicator_default() -> bool
	{
		true
	}
}
