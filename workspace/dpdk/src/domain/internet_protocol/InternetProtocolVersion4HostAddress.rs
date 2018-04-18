// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Protocol (IP) version 4 host address.
///
/// Stored internally in network byte order.
///
/// Defaults to `Unspecified` (which is the same as `Any`).
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
	
	/// Unspecified (Any) address.
	pub const Unspecified: Self = InternetProtocolVersion4HostAddress([0, 0, 0, 0]);
	
	/// Broadcast address.
	pub const Broadcast: Self = InternetProtocolVersion4HostAddress([255, 255, 255, 255]);
	
	/// From a network (big) endian u32.
	#[inline(always)]
	pub fn from_network_endian(big_endian_value: u32) -> Self
	{
		Self::from_octets(unsafe { transmute(big_endian_value) })
	}
	
	/// From octets.
	#[inline(always)]
	pub fn from_octets(octets: [u8; Self::Size]) -> Self
	{
		InternetProtocolVersion4HostAddress(octets)
	}
	
	/// From an `Ipv4Addr` to an `in_addr`.
	#[inline(always)]
	pub fn from_ipv4_addr_to_in_addr(ipv4_addr: &Ipv4Addr) -> in_addr
	{
		unsafe { transmute_copy(ipv4_addr) }
	}
	
	/// From an `Ipv4Addr`.
	#[inline(always)]
	pub fn from_ipv4_addr(ipv4_addr: &Ipv4Addr) -> Self
	{
		unsafe { transmute_copy(ipv4_addr) }
	}
	
	/// To an `in_addr`.
	#[inline(always)]
	pub fn to_in_addr(self) -> in_addr
	{
		in_addr
		{
			s_addr: self.0
		}
	}
	
	/// An a native endian u32.
	#[inline(always)]
	pub fn as_native_endian_u32(&self) -> u32
	{
		u32::from_be(self.as_network_endian_u32())
	}
	
	/// An a network (big) endian u32.
	#[inline(always)]
	pub fn as_network_endian_u32(&self) -> u32
	{
		unsafe { transmute(self.0) }
	}
	
	/// To an embedded RFC8215 globally routable (RFC8215) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_embedded_rfc8215_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To an embedded RFC6052 globally routable (RFC6052) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_embedded_rfc6052_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To a mapped (RFC4291) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_mapped_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To a deprecated compatible (RFC4291) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_deprecated_compatible_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// Is this not an unicast address?
	#[inline(always)]
	pub fn is_not_valid_unicast(self) -> bool
	{
		self.is_unspecified() ||
		self.is_loopback() ||
		self.is_multicast() ||
		self.is_documentation() ||
		self.is_broadcast()
	}
	
	/// Is this not a globally unicast address?
	#[inline(always)]
	pub fn is_not_globally_unicast_unique(self) -> bool
	{
		self.is_not_valid_unicast() || self.is_link_local() || self.is_private()
	}
	
	/// Is this a private (ie not globally routable) address?
	#[inline(always)]
	pub fn is_private(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::Private1.contains(self) || InternetProtocolVersion4NetworkAddress::Private2.contains(self) || InternetProtocolVersion4NetworkAddress::Private3.contains(self)
	}
	
	/// Is this a link local address?
	#[inline(always)]
	pub fn is_link_local(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::LinkLocal.contains(self)
	}
	
	/// Is this the unspecified address?
	#[inline(always)]
	pub fn is_unspecified(self) -> bool
	{
		self == Self::Unspecified
	}
	
	/// Is this the broadcast address?
	#[inline(always)]
	pub fn is_broadcast(self) -> bool
	{
		self == Self::Broadcast
	}
	
	/// Is this a loopback address?
	#[inline(always)]
	pub fn is_loopback(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::Loopback.contains(self)
	}
	
	/// Is this a multicast address?
	#[inline(always)]
	pub fn is_multicast(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::Multicast.contains(self)
	}
	
	/// Is this an address used for documentation and in examples?
	#[inline(always)]
	pub fn is_documentation(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::TestNet1.contains(self) || InternetProtocolVersion4NetworkAddress::TestNet2.contains(self) || InternetProtocolVersion4NetworkAddress::TestNet3.contains(self)
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
