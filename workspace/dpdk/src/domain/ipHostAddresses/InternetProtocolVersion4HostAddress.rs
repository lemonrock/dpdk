// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Protocol (IP) version 4 host address.
///
/// Stored internally in network byte order.
///
/// Defaults to `Any` (which is the same as unspecified).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolVersion4HostAddress(pub [u8; Self::Size]);

impl Default
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6HostAddress::Any
	}
}

impl InternetProtocolVersion4HostAddress
{
	/// Size of an Internet Protocol (IP) Version 4 host address.
	pub const Size: usize = 4;
	
	/// Any address.
	pub const Any: Self = InternetProtocolVersion4HostAddress([0, 0, 0, 0]);
	
	/// Broadcast address.
	pub const Broadcast: Self = InternetProtocolVersion4HostAddress([255, 255, 255, 255]);
	
	#[inline(always)]
	pub fn from_ipv4_addr_to_in_addr(ipv4_addr: &Ipv4Addr) -> in_addr
	{
		unsafe { transmute_copy(ipv4_addr) }
	}
	
	#[inline(always)]
	pub fn from_ipv4_addr(ipv4_addr: &Ipv4Addr) -> Self
	{
		unsafe { transmute_copy(ipv4_addr) }
	}
	
	#[inline(always)]
	pub fn to_in_addr(self) -> in_addr
	{
		in_addr
		{
			s_addr: self.0
		}
	}
	
	#[inline(always)]
	pub fn to_mapped_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	#[inline(always)]
	pub fn is_not_valid_unicast(self) -> bool
	{
		self.is_unspecified() ||
		self.is_loopback() ||
		self.is_multicast() ||
		self.is_documentation() ||
		self.is_broadcast()
	}
	
	#[inline(always)]
	pub fn is_not_globally_unicast_unique(self) -> bool
	{
		self.is_not_valid_unicast() || self.is_link_local() || self.is_private()
	}
	
	#[inline(always)]
	pub fn is_private(self) -> bool
	{
		IpV4NetworkAddress::Private1.contains(self) || IpV4NetworkAddress::Private2.contains(self) || IpV4NetworkAddress::Private3.contains(self)
	}
	
	#[inline(always)]
	pub fn is_link_local(self) -> bool
	{
		IpV4NetworkAddress::LinkLocal.contains(self)
	}
	
	#[inline(always)]
	pub fn is_unspecified(self) -> bool
	{
		self == Self::Any
	}
	
	#[inline(always)]
	pub fn is_broadcast(self) -> bool
	{
		self == Self::Broadcast
	}
	
	#[inline(always)]
	pub fn is_loopback(self) -> bool
	{
		IpV4NetworkAddress::Loopback.contains(self)
	}
	
	#[inline(always)]
	pub fn is_multicast(self) -> bool
	{
		IpV4NetworkAddress::Multicast.contains(self)
	}
	
	#[inline(always)]
	pub fn is_documentation(self) -> bool
	{
		IpV4NetworkAddress::TestNet1.contains(self) || IpV4NetworkAddress::TestNet2.contains(self) || IpV4NetworkAddress::TestNet3.contains(self)
	}
	
	#[inline(always)]
	fn get_first_byte(&self) -> u8
	{
		unsafe { self.0.get_unchecked(0) }
	}
	
	#[inline(always)]
	fn get_second_byte(&self) -> u8
	{
		unsafe { self.0.get_unchecked(1) }
	}
	
	#[inline(always)]
	fn get_third_byte(&self) -> u8
	{
		unsafe { self.0.get_unchecked(2) }
	}
	
	#[inline(always)]
	fn get_fourth_byte(&self) -> u8
	{
		unsafe { self.0.get_unchecked(3) }
	}
}
