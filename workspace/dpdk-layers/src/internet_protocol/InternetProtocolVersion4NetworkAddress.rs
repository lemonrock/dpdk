// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Protocol (IP) version 4 masked network address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolVersion4NetworkAddress
{
	network: InternetProtocolVersion4HostAddress,
	mask_bits: InternetProtocolVersion4MaskBits,
}

impl Display for InternetProtocolVersion4NetworkAddress
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}/{}", self.network, self.mask_bits)
	}
}

impl InternetProtocolNetworkAddress for InternetProtocolVersion4NetworkAddress
{
	type HostAddress = InternetProtocolVersion4HostAddress;
	
	type MaskBits = InternetProtocolVersion4MaskBits;
	
	#[inline(always)]
	fn network(&self) -> &Self::HostAddress
	{
		&self.network
	}
	
	#[inline(always)]
	fn mask_bits(&self) -> Self::MaskBits
	{
		self.mask_bits
	}
	
	#[inline(always)]
	fn contains(&self, internet_protocol_host_address: <InternetProtocolVersion4NetworkAddress as InternetProtocolNetworkAddress>::HostAddress) -> bool
	{
		internet_protocol_host_address.as_network_endian() & (self.mask_bits as u32) == self.network.as_network_endian()
	}
	
	#[inline(always)]
	fn new(network: Self::HostAddress, mask_bits: <<InternetProtocolVersion4NetworkAddress as InternetProtocolNetworkAddress>::HostAddress as InternetProtocolHostAddress>::MaskBits) -> Self
	{
		Self
		{
			network,
			mask_bits,
		}
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
	
	/// Is this address invalid for this network because it ends in a zero or is the reserved network broadcast address?
	#[inline(always)]
	pub fn is_invalid_unicast_for_network(&self, internet_protocol_host_address: <internet_protocol::InternetProtocolVersion4NetworkAddress as InternetProtocolNetworkAddress>::HostAddress) -> bool
	{
		if self.contains(internet_protocol_host_address)
		{
			self.is_first_address(internet_protocol_host_address) || self.is_broadcast_address(internet_protocol_host_address)
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	fn is_first_address(&self, internet_protocol_host_address: <InternetProtocolVersion4NetworkAddress as InternetProtocolNetworkAddress>::HostAddress) -> bool
	{
		self.network.as_network_endian() == internet_protocol_host_address.as_network_endian()
	}
	
	#[inline(always)]
	fn is_broadcast_address(&self, internet_protocol_host_address: <InternetProtocolVersion4NetworkAddress as InternetProtocolNetworkAddress>::HostAddress) -> bool
	{
		let inverse_mask_bits = !(self.mask_bits as u32);
		internet_protocol_host_address.as_network_endian() & inverse_mask_bits == inverse_mask_bits
	}
}
