// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Protocol (IP) version 6 masked network address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolVersion6NetworkAddress
{
	network: InternetProtocolVersion6HostAddress,
	mask_bits: InternetProtocolVersion6MaskBits,
}

impl Display for InternetProtocolVersion6NetworkAddress
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}/{}", self.network, self.mask_bits)
	}
}

impl InternetProtocolNetworkAddress for InternetProtocolVersion6NetworkAddress
{
	type HostAddress = InternetProtocolVersion6HostAddress;
	
	type MaskBits = InternetProtocolVersion6MaskBits;
	
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
	fn contains(&self, internet_protocol_host_address: Self::HostAddress) -> bool
	{
		internet_protocol_host_address.as_network_endian() & (self.mask_bits as u128) == self.network.as_network_endian()
	}
	
	#[inline(always)]
	fn new(network: Self::HostAddress, mask_bits: <<InternetProtocolVersion6NetworkAddress as InternetProtocolNetworkAddress>::HostAddress as InternetProtocolHostAddress>::MaskBits) -> Self
	{
		Self
		{
			network,
			mask_bits,
		}
	}
}

impl InternetProtocolVersion6NetworkAddress
{
	/// 2000::/3 prefix.
	///
	/// Can include non-globally routable 'carve outs' such as documentation and unique local unicast.
	pub const GloballyRoutablePrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_3,
	};
	
	/// RFC 8215: Globally routable internet protocol (IP) version 4 addresses.
	///
	/// Uses 'well-known prefix' of `64:ff9b:1::/96`.
	pub const GloballyRoutableRfc8215InternetProtocolVersion4AddressPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_96,
	};
	
	/// RFC 6052: Globally routable internet protocol (IP) version 4 addresses.
	///
	/// Uses 'well-known prefix' of `64:ff9b::/96`.
	pub const GloballyRoutableRfc6052InternetProtocolVersion4AddressPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_96,
	};
	
	/// RFC 4291: Mapped internet protocol (IP) version 4 addresses.
	///
	/// Uses prefix `::FFFF::/96`.
	pub const MappedInternetProtocolVersion4AddressPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_96,
	};
	
	/// RFC 4291: Deprecated embedded internet protocol (IP) version 4 addresses.
	///
	/// Uses prefix `::/96`.
	///
	/// Note that the prefix alone does not imply a valid address.
	pub const DeprecatedEmbeddedInternetProtocolVersion4AddressPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_96,
	};
	
	/// RFC 3056: 6to4.
	///
	/// Uses prefix `2002::/16`.
	pub const _6to4Prefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x20, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_16,
	};
	
	/// RFC 7532: Direct Delegation AS112 Service.
	///
	/// Uses prefix `2620:4f:8000::/48`.
	pub const DirectDelegationAs112ServicePrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x26, 0x20, 0x4f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_48,
	};
	
	/// RFC 6666: Discard only.
	///
	/// Uses prefix `100::/64`.
	pub const DiscardOnlyPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_64,
	};
	
	/// RFC 4193 & RFC 8190.
	///
	/// Even though this prefix falls within those that are globally routed, these addresses are not globally routed...
	///
	/// Uses prefix` fc00::/7` (although bit 8 is (currently) always specified to be 1).
	pub const UniqueLocalUnicastPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0xFC, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_7,
	};
	
	/// RFC 4291.
	///
	/// Uses prefix `fe80::/10` *but* since the next 54 bits *must* always be zero, use `Self::LinkLocalUnicastPragmaticPrefix` in practice.
	#[inline(always)]
	pub const LinkLocalUnicastPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0xFE, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_10,
	};
	
	/// Pragmatic implementation of RFC 4291.
	///
	/// Uses a prefix of prefix `fe80::/64` since the 54 bits after the official prefix of `fe80/10` must always be zero.
	#[inline(always)]
	pub const LinkLocalUnicastPragmaticPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0xFE, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_64,
	};
	
	//noinspection SpellCheckingInspection
	/// RFC 2928.
	///
	/// Uses prefix `2001::/23`.
	pub const AssignedIetfProtocolPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_23,
	};
	
	/// RFC 4380 and RFC 8190.
	///
	/// Uses prefix `2001::/32`.
	pub const TeredoPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_32,
	};
	
	/// RFC 5180 and RFC Errata 1752.
	///
	/// Uses prefix `2001:2::/48`.
	pub const BenchmarkingPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_48,
	};
	
	/// RFC 7450.
	///
	/// Uses prefix `2001:3::/32`.
	pub const AmtPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_32,
	};
	
	/// RFC 7535.
	///
	/// Uses prefix `2001:4:112::/48`.
	pub const As112V6Prefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x04, 0x01, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_48,
	};
	
	/// RFC 7954.
	///
	/// Uses prefix `2001:5::/32`.
	pub const EidSpaceForLispPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_32,
	};
	
	/// RFC 4843.
	///
	/// Uses prefix `2001:10::/28`.
	pub const DeprecatedOrchidPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_28,
	};
	
	/// RFC 7343.
	///
	/// Uses prefix `2001:20::/28`.
	pub const OrchidV2Prefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x02, 0x01, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_28,
	};
	
	/// RFC 4291 (deprecated in RFC 3879, defined originally in RFC 3513 which itself obsoletes RFC 2373).
	///
	/// Uses prefix `fec0::/10`.
	pub const DeprecatedSiteLocalUnicastPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0xFE, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_10,
	};
	
	/// Originally RFC 4291 and RFC 4007, updated by RFC 7346.
	///
	/// Uses prefix `ff::/8`.
	pub const MulticastPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_8,
	};
	
	/// RFC 4291.
	///
	/// Uses prefix `ff02:0:0:0:0:1:ff00::/104`.
	pub const MulticastSolicitedNodePrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0xFF, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xFF, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_104,
	};
	
	/// RFC 3849.
	///
	/// Uses prefix `2001:db8::/16`.
	pub const DocumentationPrefix: Self = Self
	{
		network: InternetProtocolVersion6HostAddress([0x20, 0x01, 0x0D, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
		mask_bits: InternetProtocolVersion6MaskBits::_16,
	};
}
