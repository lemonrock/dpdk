// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet protocol (IP) version 4 payload of address resolution protocol (ARP) packet.
#[repr(C, packed)]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[allow(missing_docs)]
	pub sender_hardware_address: MediaAccessControlAddress,
	
	#[allow(missing_docs)]
	pub sender_protocol_address: InternetProtocolVersion4HostAddress,
	
	#[allow(missing_docs)]
	pub target_hardware_address: MediaAccessControlAddress,
	
	#[allow(missing_docs)]
	pub target_protocol_address: InternetProtocolVersion4HostAddress,
}

impl Display for AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Into<arp_ipv4> for AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn into(self) -> arp_ipv4
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a arp_ipv4> for &'a AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn into(self) -> &'a arp_ipv4
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a mut arp_ipv4> for &'a mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn into(self) -> &'a mut arp_ipv4
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<NonNull<arp_ipv4>> for &'a mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn into(self) -> NonNull<arp_ipv4>
	{
		unsafe { NonNull::new_unchecked(self as *mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload as *mut arp_ipv4) }
	}
}

impl<'a> Into<*const arp_ipv4> for &'a AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn into(self) -> *const arp_ipv4
	{
		self as *const AddressResolutionProtocolPacketInternetProtocolVersion4Payload as *const _
	}
}

impl<'a> Into<*mut arp_ipv4> for &'a mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn into(self) -> *mut arp_ipv4
	{
		self as *mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload as *mut _
	}
}

impl From<arp_ipv4> for AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn from(value: arp_ipv4) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a arp_ipv4> for &'a AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn from(value: &'a arp_ipv4) -> &'a AddressResolutionProtocolPacketInternetProtocolVersion4Payload
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a mut arp_ipv4> for &'a mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload
{
	#[inline(always)]
	fn from(value: &'a mut arp_ipv4) -> &'a mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload
	{
		unsafe { transmute(value) }
	}
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
