// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Protocol (IP) version 4 masked network address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolVersion4NetworkAddress
{
	pub network: InternetProtocolVersion4HostAddress,
	pub mask_bits: InternetProtocolVersion4MaskBits,
}

impl InternetProtocolNetworkAddress for InternetProtocolVersion4NetworkAddress
{
	type InternetProtocolHostAddress = InternetProtocolVersion4HostAddress;
	
	#[inline(always)]
	fn network(&self) -> &Self::InternetProtocolHostAddress
	{
		&self.network
	}
	
	#[inline(always)]
	fn mask_bits_as_depth(&self) -> u8
	{
		let mask_bits = self.mask_bits as u32;
		if cfg!(target_endian = "little")
		{
			mask_bits.count_ones()
		}
		else
		{
			(!mask_bits).trailing_zeros()
		}
	}
	
	#[inline(always)]
	fn contains(&self, internet_protocol_host_address: Self::InternetProtocolHostAddress) -> bool
	{
		internet_protocol_host_address.as_network_endian_u32() & (self.mask_bits as u32) == self.network.as_network_endian_u32()
	}
}

impl InternetProtocolVersion4NetworkAddress
{
	/// RFC 1122.
	///
	/// Equivalent to a Class A network.
	pub const Loopback: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([127, 0, 0, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_8,
	};
	
	/// Multicast.
	pub const Multicast: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([224, 0, 0, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_4,
	};
	
	/// RFC 5737.
	///
	/// Equivalent to a Class C network.
	pub const TestNet1: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([192, 0, 2, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_24,
	};
	
	/// RFC 5737.
	///
	/// Equivalent to a Class C network.
	pub const TestNet2: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([198, 51, 100, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_24,
	};
	
	/// RFC 5737.
	///
	/// Equivalent to a Class C network.
	pub const TestNet3: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([203, 0, 113, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_24,
	};
	
	/// RFC 1918.
	///
	/// The private Class A network.
	pub const Private1: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([10, 0, 0, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_8,
	};
	
	/// RFC 1918.
	///
	/// The private Class B network.
	pub const Private2: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([172, 16, 0, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_12,
	};
	
	/// RFC 1918.
	///
	/// The private Class C network.
	pub const Private3: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([192, 168, 0, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_16,
	};
	
	/// RFC 3927.
	///
	/// The link-local Class B network.
	pub const LinkLocal: Self = Self
	{
		network: InternetProtocolVersion4HostAddress([169, 254, 0, 0]),
		mask_bits: InternetProtocolVersion4MaskBits::_16,
	};
}
