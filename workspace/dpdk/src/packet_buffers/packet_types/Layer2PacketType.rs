// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Layer 2 packet type.
///
/// If the packet is a tunneled packet, then this is known as the Outer Layer 2 packet type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Layer2PacketType
{
	/// Either:-
	///
	/// * Ignore this packet.
	/// * Hardware categorisation hasn't happened.
	///
	/// It seems possible that some drivers don't set `RTE_PTYPE_L2_ETHER` even on ethernet packets.
	Unknown,
	
	/// Ether packet; may be further categorised.
	///
	/// Most drivers, excluding Intel's, do not categorise further.
	Ethernet(Option<CategorisedLayer2PacketType>),
}

impl Layer2PacketType
{
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 3:0 (0x0F) are significant.
	#[inline(always)]
	pub fn from_packet_buffer_packet_type(packet_type: u32) -> Self
	{
		use self::Layer2PacketType::*;
		use self::CategorisedLayer2PacketType::*;
		
		match packet_type & RTE_PTYPE_L2_MASK
		{
			RTE_PTYPE_UNKNOWN => Unknown,
			
			RTE_PTYPE_L2_ETHER => Ethernet(None),
			
			RTE_PTYPE_L2_ETHER_TIMESYNC => Ethernet(Some(Ieee1588TimeSync)),
			
			RTE_PTYPE_L2_ETHER_ARP => Ethernet(Some(AddressResolutionProtocol)),
			
			RTE_PTYPE_L2_ETHER_LLDP => Ethernet(Some(LinkLayerDiscoveryProtocol)),
			
			RTE_PTYPE_L2_ETHER_NSH => Ethernet(Some(NetworkServiceHeader)),
			
			// Valid only if Virtual LAN stripping is disabled.
			RTE_PTYPE_L2_ETHER_VLAN => Ethernet(Some(VirtualLan)),
			
			// ?Valid only if Virtual LAN stripping is disabled?
			RTE_PTYPE_L2_ETHER_QINQ => Ethernet(Some(QinQVirtualLan)),
			
			RTE_PTYPE_L2_ETHER_PPPOE => Ethernet(Some(PPPoE)),
			
			invalid_or_introduced_after_this_code_was_written @ _ => Ethernet(Some(InvalidOrIntroducedAfterThisCodeWasWritten(invalid_or_introduced_after_this_code_was_written)))
		}
	}
	
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 19:16 are significant.
	#[inline(always)]
	pub fn inner_layer_2_for_tunnel_from_packet_buffer_packet_type(packet_type: u32) -> Self
	{
		use self::Layer2PacketType::*;
		use self::CategorisedLayer2PacketType::*;
		
		match packet_type & RTE_PTYPE_INNER_L2_MASK
		{
			0 => Unknown,
			
			RTE_PTYPE_INNER_L2_ETHER => Ethernet(None),
			
			RTE_PTYPE_INNER_L2_ETHER_VLAN => Ethernet(Some(VirtualLan)),
			
			RTE_PTYPE_INNER_L2_ETHER_QINQ => Ethernet(Some(QinQVirtualLan)),
			
			invalid_or_introduced_after_this_code_was_written @ _ => Ethernet(Some(InvalidOrIntroducedAfterThisCodeWasWritten(invalid_or_introduced_after_this_code_was_written)))
		}
	}
}
