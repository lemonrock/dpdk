// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Layer 4 packet type.
///
/// If the packet is a tunneled packet, then this is known as the Outer Layer 4 packet type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TunnelPacketType
{
	/// Probably not a tunnel packet type.
	Uncategorised,
	
	/// Internet protocol (IP) in internet protocol (IP) tunnel.
	InternetProtocol,
	
	/// Generic Routing Encapsulation (GRE).
	GenericRoutingEncapsulation,
	
	/// Virtual eXtensible Local Area Network (VxLAN).
	VirtualExtensibleLocalAreaNetwork,
	
	/// Network Virtualization using Generic Routing Encapsulation (NVGRE).
	NetworkVirtualizationUsingGenericRoutingEncapsulation,
	
	/// Generic Network Virtualization Encapsulation (GENEVE).
	GenericNetworkVirtualizationEncapsulation,
	
	/// ?
	TeredoOrGenericRoutingEncapsulationOrVirtualExtensibleLocalAreaNetwork,
	
	/// GPRS Tunneling Protocol control (GTP-C).
	GprsTunnelingProtocolControl,
	
	/// GPRS Tunneling Protocol user data (GTP-U).
	GprsTunnelingProtocolUserData,
	
	/// IP Encapsulating Security Payload (ESP).
	///
	/// Part of IPsec.
	InternetProtocolEncapsulatingSecurityPayload,
	
	/// Layer 2 Tunneling Protocol (L2TP).
	Layer2TunnelingProtocol,
	
	/// Invalid or introduced after this code was written.
	InvalidOrIntroducedAfterThisCodeWasWritten(u32),
}

impl TunnelPacketType
{
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 15:12 (0xF000) are significant.
	#[inline(always)]
	pub fn from_packet_buffer_packet_type(packet_type: u32) -> Self
	{
		use self::TunnelPacketType::*;
		
		match packet_type & RTE_PTYPE_TUNNEL_MASK
		{
			0 => Uncategorised,
			
			RTE_PTYPE_TUNNEL_IP => InternetProtocol,
			
			RTE_PTYPE_TUNNEL_GRE => GenericRoutingEncapsulation,
			
			RTE_PTYPE_TUNNEL_VXLAN => VirtualExtensibleLocalAreaNetwork,
			
			RTE_PTYPE_TUNNEL_NVGRE => NetworkVirtualizationUsingGenericRoutingEncapsulation,
			
			RTE_PTYPE_TUNNEL_GENEVE => GenericNetworkVirtualizationEncapsulation,
			
			RTE_PTYPE_TUNNEL_GRENAT => TeredoOrGenericRoutingEncapsulationOrVirtualExtensibleLocalAreaNetwork,
			
			RTE_PTYPE_TUNNEL_GTPC => GprsTunnelingProtocolControl,
			
			RTE_PTYPE_TUNNEL_GTPU => GprsTunnelingProtocolUserData,
			
			RTE_PTYPE_TUNNEL_ESP => InternetProtocolEncapsulatingSecurityPayload,
			
			RTE_PTYPE_TUNNEL_L2TP => Layer2TunnelingProtocol,
			
			invalid_or_introduced_after_this_code_was_written @ _ => InvalidOrIntroducedAfterThisCodeWasWritten(InvalidOrIntroducedAfterThisCodeWasWritten(invalid_or_introduced_after_this_code_was_written))
		}
	}
}
