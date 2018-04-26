// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Layer 4 packet type.
///
/// If the packet is a tunneled packet, then this is known as the Outer Layer 4 packet type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Layer4PacketType
{
	/// Either the driver did not categorise this packet or the layer 4 data structure is absent.
	UncategorisedOrAbsent,
	
	/// Transmission Control Protocol (TCP).
	TransmissionControlProtocol,
	
	/// User Datagram Protocol (UCP).
	UserDatagramProtocol,
	
	/// Stream Control Transmission Protocol (SCTP).
	StreamControlTransmissionProtocol,
	
	/// Internet Control Message Protocol (ICMP).
	///
	/// Only used on networks supporting internet protocol (IP) version 4.
	InternetControlMessageProtocol,
	
	/// A fragmented internet protocol (IP) version 4 or version 6 packet.
	///
	/// Will never be the first fragment.
	///
	/// May not necessarily be a fragment of a TCP, UDP, SCTP or ICMP packet.
	Fragmented,
	
	/// A internet protocol (IP) version 4 or version 6 packet which is:-
	///
	/// * not TCP, UDP, SCTP or ICMP;
	/// * not a fragment
	OtherNotAFragment,
	
	/// Invalid or introduced after this code was written.
	InvalidOrIntroducedAfterThisCodeWasWritten(u32),
}

impl Layer4PacketType
{
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 11:8 (0x0F00) are significant.
	#[inline(always)]
	pub fn from_packet_buffer_packet_type(packet_type: u32) -> Self
	{
		use self::Layer4PacketType::*;
		
		match packet_type & RTE_PTYPE_L4_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_L4_TCP => TransmissionControlProtocol,
			
			RTE_PTYPE_L4_UDP => UserDatagramProtocol,
			
			RTE_PTYPE_L4_FRAG => Fragmented,
			
			RTE_PTYPE_L4_SCTP => StreamControlTransmissionProtocol,
			
			RTE_PTYPE_L4_ICMP => InternetControlMessageProtocol,
			
			RTE_PTYPE_L4_NONFRAG => OtherNotAFragment,
			
			invalid_or_introduced_after_this_code_was_written @ _ => InvalidOrIntroducedAfterThisCodeWasWritten(InvalidOrIntroducedAfterThisCodeWasWritten(invalid_or_introduced_after_this_code_was_written))
		}
	}
	
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 27:24 are significant.
	#[inline(always)]
	pub fn inner_layer_4_for_tunnel_from_packet_buffer_packet_type(packet_type: u32) -> Self
	{
		use self::Layer4PacketType::*;
		
		match packet_type & RTE_PTYPE_INNER_L4_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_INNER_L4_TCP => TransmissionControlProtocol,
			
			RTE_PTYPE_INNER_L4_UDP => UserDatagramProtocol,
			
			RTE_PTYPE_INNER_L4_FRAG => Fragmented,
			
			RTE_PTYPE_INNER_L4_SCTP => StreamControlTransmissionProtocol,
			
			RTE_PTYPE_INNER_L4_ICMP => InternetControlMessageProtocol,
			
			RTE_PTYPE_INNER_L4_NONFRAG => OtherNotAFragment,
			
			invalid_or_introduced_after_this_code_was_written @ _ => InvalidOrIntroducedAfterThisCodeWasWritten(InvalidOrIntroducedAfterThisCodeWasWritten(invalid_or_introduced_after_this_code_was_written))
		}
	}
}
