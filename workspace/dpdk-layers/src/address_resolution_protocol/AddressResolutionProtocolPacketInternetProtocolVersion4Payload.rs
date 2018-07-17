// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet protocol (IP) version 4 payload of address resolution protocol (ARP) packet.
#[repr(C, packed)]
pub struct AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	sender_hardware_address: MediaAccessControlAddress,
	sender_protocol_address: InternetProtocolVersion4HostAddress,
	target_hardware_address: MediaAccessControlAddress,
	target_protocol_address: InternetProtocolVersion4HostAddress,
}

impl AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	/// ARP probe.
	#[inline(always)]
	pub fn matches_a_request_probe(&self) -> bool
	{
		self.sender_protocol_address.is_unspecified()
	}
	
	/// Common.
	#[inline(always)]
	pub fn matches_a_gratuitous_request_announcement(&self) -> bool
	{
		self.sender_protocol_address == self.target_protocol_address && self.target_hardware_address.is_zero()
	}
	
	/// Rare.
	#[inline(always)]
	pub fn matches_a_gratuitous_reply_announcement(&self) -> bool
	{
		self.sender_protocol_address == self.target_protocol_address && self.target_hardware_address == self.sender_hardware_address
	}
}
