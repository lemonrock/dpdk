// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Layer 3 packet type.
///
/// If the packet is a tunneled packet, then this is known as the Outer Layer 3 packet type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Layer3PacketType
{
	/// Either the driver did not categorise this packet or the layer 3 data structure is absent.
	UncategorisedOrAbsent,
	
	/// Internet protocol (IP) version 4.
	///
	/// EtherType 0x0800.
	InternetProtocolVersion4(CategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType),
	
	/// Internet protocol (IP) version 6.
	///
	/// EtherType 0x86DD.
	InternetProtocolVersion6(CategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType),
	
	/// Invalid or introduced after this code was written.
	InvalidOrIntroducedAfterThisCodeWasWritten(u32),
}

impl Layer3PacketType
{
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 7:4 (0xF0) are significant.
	#[inline(always)]
	pub fn from_packet_buffer_packet_type(packet_type: u32) -> Self
	{
		use self::Layer3PacketType::*;
		use self::CategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType::*;
		
		match packet_type & RTE_PTYPE_L3_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_L3_IPV4 => InternetProtocolVersion4(NotPresent),
			
			RTE_PTYPE_L3_IPV4_EXT => InternetProtocolVersion4(Present),
			
			RTE_PTYPE_L3_IPV4_EXT_UNKNOWN => InternetProtocolVersion4(PresentAndUnrecognised),
			
			RTE_PTYPE_L3_IPV6 => InternetProtocolVersion6(NotPresent),
			
			RTE_PTYPE_L3_IPV6_EXT => InternetProtocolVersion6(Present),
			
			RTE_PTYPE_L3_IPV6_EXT_UNKNOWN => InternetProtocolVersion6(PresentAndUnrecognised),
			
			invalid_or_introduced_after_this_code_was_written @ _ => InvalidOrIntroducedAfterThisCodeWasWritten(InvalidOrIntroducedAfterThisCodeWasWritten(invalid_or_introduced_after_this_code_was_written))
		}
	}
	
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 23:20 are significant.
	#[inline(always)]
	pub fn inner_layer_3_for_tunnel_from_packet_buffer_packet_type(packet_type: u32) -> Self
	{
		use self::Layer2PacketType::*;
		use self::CategorisedLayer2PacketType::*;
		
		match packet_type & RTE_PTYPE_INNER_L3_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_INNER_L3_IPV4 => InternetProtocolVersion4(NotPresent),
			
			RTE_PTYPE_INNER_L3_IPV4_EXT => InternetProtocolVersion4(Present),
			
			RTE_PTYPE_INNER_L3_IPV4_EXT_UNKNOWN => InternetProtocolVersion4(PresentAndUnrecognised),
			
			RTE_PTYPE_INNER_L3_IPV6 => InternetProtocolVersion6(NotPresent),
			
			RTE_PTYPE_INNER_L3_IPV6_EXT => InternetProtocolVersion6(Present),
			
			RTE_PTYPE_INNER_L3_IPV6_EXT_UNKNOWN => InternetProtocolVersion6(PresentAndUnrecognised),
			
			invalid_or_introduced_after_this_code_was_written @ _ => InvalidOrIntroducedAfterThisCodeWasWritten(InvalidOrIntroducedAfterThisCodeWasWritten(invalid_or_introduced_after_this_code_was_written))
		}
	}
}
