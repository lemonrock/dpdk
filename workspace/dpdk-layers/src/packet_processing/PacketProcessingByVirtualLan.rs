// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet processing configuration by Virtual LAN.
#[derive(Debug)]
pub struct PacketProcessingByVirtualLan<PPDO: PacketProcessingDropObserver>
{
	/// Outer QinQ Virtual LAN.
	outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), PacketProcessingForQinQVirtualLan<PPDO>>,
	
	/// Inner 802.1Q Virtual LAN.
	inner: HashMap<VirtualLanIdentifier, PacketProcessing<PPDO>>,
	
	/// No virtual LANs.
	none: PacketProcessing<PPDO>,
}

impl<PPDO: PacketProcessingDropObserver> PacketProcessingByVirtualLan<PPDO>
{
	#[inline(always)]
	#[inline(always)]
	pub(crate) fn dropped_packet(&self, reason: PacketProcessingDropReason)
	{
		self.none.dropped_packet(reason)
	}
	
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_outer_virtual_lan(&self, outer_virtual_lan_identifier: Option<VirtualLanIdentifier>, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&PacketProcessingForQinQVirtualLan<PPDO>>
	{
		self.outer.get(&(inner_virtual_lan_identifier, outer_virtual_lan_identifier))
	}
	
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_inner_virtual_lan(&self, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&PacketProcessing<PPDO>>
	{
		match inner_virtual_lan_identifier
		{
			None => Some(&self.none),
			Some(ref virtual_lan_identifier) => self.inner.get(virtual_lan_identifier),
		}
	}
}
