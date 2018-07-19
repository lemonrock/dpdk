// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait abstracting the similarities between internet protocol (IP) version 4 and version 6 host addresses.
pub trait InternetProtocolHostAddress: Sized + Debug + Display + PartialOrd + Ord + PartialEq + Eq + Hash + Serialize + Clone
{
	/// eg `u128`.
	type BigEndianValue: Debug;
	
	/// eg `Ipv6Addr`.
	type RustAddress: Debug;
	
	/// eg `in6_addr`.
	type LibCAddress;
	
	/// eg `InternetProtocolVersion6MaskBits`.
	type MaskBits: InternetProtocolMaskBits;
	
	/// eg `[u8; Self::Size]`.
	type Octets: Sized;
	
	/// Size of an Internet Protocol (IP) host address.
	const Size: usize;
	
	/// Size of an Internet Protocol (IP) host address, as an u8.
	const SizeU8: u8;
	
	/// From a network (big) endian u32 or u128.
	#[inline(always)]
	fn from_network_endian(big_endian_value: Self::BigEndianValue) -> Self
	{
		Self::from_octets(unsafe { transmute(big_endian_value) })
	}
	
	/// From octets.
	#[inline(always)]
	fn from_octets(octets: Self::Octets) -> Self;
	
	/// eg from an `Ipv6Addr` to an `in6_addr`.
	#[inline(always)]
	fn from_rust_address_to_libc_address(rust_address: &Self::RustAddress) -> Self::LibCAddress;
	
	/// eg from an `Ipv6Addr`.
	#[inline(always)]
	fn from_rust_address(rust_address: &Self::RustAddress) -> Self;
	
	/// eg to an `Ipv6Addr`.
	#[inline(always)]
	fn to_rust_address(&self) -> Self::RustAddress;
	
	/// eg to an `in6_addr`.
	#[inline(always)]
	fn to_libc_address(self) -> Self::LibCAddress;
	
	/// As an a native endian, eg `u128`.
	#[inline(always)]
	fn as_native_endian(&self) -> Self::BigEndianValue;
	
	/// As an a network (big) endian, eg `u128`.
	#[inline(always)]
	fn as_network_endian(&self) -> Self::BigEndianValue;
	
	/// Tries to convert to a media access control address.
	///
	/// Will fail for some address classes and types; ideally an address is private (version 4) or link-local (version 6).
	#[inline(always)]
	fn to_media_access_control_address(&self) -> Result<MediaAccessControlAddress, ()>;
}
