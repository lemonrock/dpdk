// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Protocol (IP) version 4 host address.
///
/// Stored internally in network byte order.
///
/// Defaults to `Unspecified` (which is the same as `Any`).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(packed)]
pub struct InternetProtocolVersion4HostAddress(pub [u8; InternetProtocolVersion4HostAddress::Size]);

impl Display for InternetProtocolVersion4HostAddress
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}.{}.{}.{}", self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte())
	}
}

impl Debug for InternetProtocolVersion4HostAddress
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}.{}.{}.{}", self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte())
	}
}

impl Default for InternetProtocolVersion4HostAddress
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Unspecified
	}
}

/// A trait abstracting the similarities between internet protocol (IP) version 4 and version 6 host addresses.
impl InternetProtocolHostAddress for InternetProtocolVersion4HostAddress
{
	type BigEndianValue = u32;
	
	type RustAddress = Ipv4Addr;
	
	type LibCAddress = in_addr;
	
	type MaskBits = InternetProtocolVersion4MaskBits;
	
	type Octets = [u8; 4];
	
	const Size: usize = 4;
	
	const SizeU8: u8 = 4;
	
	#[inline(always)]
	fn from_octets(octets: Self::Octets) -> Self
	{
		InternetProtocolVersion4HostAddress(octets)
	}
	
	#[inline(always)]
	fn from_rust_address_to_libc_address(rust_address: &Self::RustAddress) -> Self::LibCAddress
	{
		unsafe { transmute_copy(rust_address) }
	}
	
	#[inline(always)]
	fn from_rust_address(rust_address: &Self::RustAddress) -> Self
	{
		unsafe { transmute_copy(rust_address) }
	}
	
	#[inline(always)]
	fn to_rust_address(&self) -> Self::RustAddress
	{
		unsafe { transmute_copy(self) }
	}
	
	#[inline(always)]
	fn to_libc_address(self) -> Self::LibCAddress
	{
		in_addr
		{
			s_addr: unsafe { transmute(self.0) },
		}
	}
	
	#[inline(always)]
	fn as_native_endian(&self) -> Self::BigEndianValue
	{
		u32::from_be(self.as_network_endian())
	}
	
	#[inline(always)]
	fn as_network_endian(&self) -> Self::BigEndianValue
	{
		unsafe { transmute(self.0) }
	}
	
	#[inline(always)]
	fn to_media_access_control_address(&self) -> Result<MediaAccessControlAddress, ()>
	{
		MediaAccessControlAddress::from_private_internet_protocol_version_4_host_address(self)
	}
}

impl InternetProtocolVersion4HostAddress
{
	/// Unspecified (Any) address.
	pub const Unspecified: Self = InternetProtocolVersion4HostAddress([0, 0, 0, 0]);
	
	/// Broadcast address.
	pub const Broadcast: Self = InternetProtocolVersion4HostAddress([255, 255, 255, 255]);
	
	/// To an embedded RFC 8215 globally routable (RFC 8215) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_embedded_rfc8215_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To an embedded RFC 6052 globally routable (RFC 6052) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_embedded_rfc6052_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To a mapped (RFC 4291) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_mapped_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To a deprecated compatible (RFC 4291) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_deprecated_compatible_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// Is this a valid unicast address?
	#[inline(always)]
	pub fn is_valid_unicast(self) -> bool
	{
		!self.is_not_valid_unicast()
	}
	
	/// Is this not a valid unicast address?
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
	
	/// Is this not the broadcast address?
	#[inline(always)]
	pub fn is_not_broadcast(self) -> bool
	{
		self != Self::Broadcast
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
	
	/// Is this a multicast address?
	#[inline(always)]
	pub fn is_not_multicast(self) -> bool
	{
		!self.is_multicast()
	}
	
	/// Is this an address used for documentation and in examples?
	#[inline(always)]
	pub fn is_documentation(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::TestNet1.contains(self) || InternetProtocolVersion4NetworkAddress::TestNet2.contains(self) || InternetProtocolVersion4NetworkAddress::TestNet3.contains(self)
	}
	
	/// Are the lower 23 bits a match?
	///
	/// Used for multicast addresses.
	#[inline(always)]
	pub fn has_lower_23_bits(self, lower_23_bits: &[u8; 3]) -> bool
	{
		self.get_second_byte() & 0b0111_1111 == lower_23_bits[0] && self.get_third_byte() == lower_23_bits[1] && self.get_fourth_byte() == lower_23_bits[2]
	}
	
	/// Are the lower 23 bits a match?
	///
	/// Used for multicast addresses.
	#[inline(always)]
	pub fn does_not_have_lower_23_bits(self, lower_23_bits: &[u8; 3]) -> bool
	{
		!self.has_lower_23_bits(lower_23_bits)
	}
	
	#[inline(always)]
	fn get_first_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(0) }
	}
	
	#[inline(always)]
	fn get_second_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(1) }
	}
	
	#[inline(always)]
	fn get_third_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(2) }
	}
	
	#[inline(always)]
	fn get_fourth_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(3) }
	}
}
