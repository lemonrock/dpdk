// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet processing configuration by Virtual LAN.
#[derive(Debug)]
#[derive(Serialization, Deserialization)]
pub struct PacketProcessingConfigurationByVirtualLan
{
	/// Outer QinQ Virtual LAN.
	pub outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), PacketProcessingConfigurationForQinQVirtualLan>,
	
	/// Inner 802.1Q Virtual LAN.
	pub inner: HashMap<VirtualLanIdentifier, PacketProcessingConfiguration>,
	
	/// No virtual LANs.
	pub none: PacketProcessingConfiguration,
}

impl PacketProcessingConfigurationByVirtualLan
{
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_outer_virtual_lan(&self, outer_virtual_lan_identifier: Option<VirtualLanIdentifier>, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&PacketProcessingConfigurationForQinQVirtualLan>
	{
		self.outer.get(&(inner_virtual_lan_identifier, outer_virtual_lan_identifier))
	}
	
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_inner_virtual_lan(&self, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&PacketProcessingConfiguration>
	{
		match inner_virtual_lan_identifier
		{
			None => &self.none,
			Some(ref virtual_lan_identifier) => self.inner.get(virtual_lan_identifier),
		}
	}
}
