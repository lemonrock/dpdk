// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct IpV4NetworkAddress
{
	#[serde(serialize_with = "IpV4NetworkAddress::serde_serialize_network", deserialize_with = "IpV4NetworkAddress::serde_deserialize_network")] pub network: IpV4HostAddress,
	pub maskBits: IpV4MaskBits,
}

impl IpNetworkAddress for IpV4NetworkAddress
{
	type IpHostAddress = IpV4HostAddress;
	
	#[inline(always)]
	fn network(&self) -> &Self::IpHostAddress
	{
		&self.network
	}
	
	#[inline(always)]
	fn maskBitsAsDepth(&self) -> u8
	{
		self.maskBits as u8
	}
}

impl IpV4NetworkAddress
{
	// RFC 1122
	pub const Loopback: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(127, 0, 0, 0),
		maskBits: IpV4MaskBits::_8,
	};
	
	pub const Multicast: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(224, 0, 0, 0),
		maskBits: IpV4MaskBits::_4,
	};
	
	// RFC 5737
	pub const TestNet1: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(192, 0, 2, 0),
		maskBits: IpV4MaskBits::_24,
	};
	
	// RFC 5737
	pub const TestNet2: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(198, 51, 100, 0),
		maskBits: IpV4MaskBits::_24,
	};
	
	// RFC 5737
	pub const TestNet3: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(203, 0, 113, 0),
		maskBits: IpV4MaskBits::_24,
	};
	
	// RFC 1918
	pub const Private1: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(10, 0, 0, 0),
		maskBits: IpV4MaskBits::_8,
	};
	
	// RFC 1918
	pub const Private2: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(172, 16, 0, 0),
		maskBits: IpV4MaskBits::_12,
	};
	
	// RFC 1918
	pub const Private3: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(192, 168, 0, 0),
		maskBits: IpV4MaskBits::_16,
	};
	
	// RFC 3927
	pub const LinkLocal: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: ipv4HostAddressFromNumbers(169, 254, 0, 0),
		maskBits: IpV4MaskBits::_16,
	};
	
	#[inline(always)]
	pub fn contains(&self, ipV4HostAddress: IpV4HostAddress) -> bool
	{
		ipV4HostAddress & self.maskBits.asMask() == self.network
	}
	
	fn serde_serialize_network<S: Serializer>(value: &IpV4HostAddress, serializer: S) -> Result<S::Ok, S::Error>
	{
		Ipv4Addr::from(*value).serialize(serializer)
	}
	
	fn serde_deserialize_network<D: Deserializer>(deserializer: D) -> Result<IpV4HostAddress, D::Error>
	{
		let ipV4Addr = Ipv4Addr::deserialize(deserializer)?;
		let octets = ipV4Addr.octets();
		Ok(ipv4HostAddressFromNumbers(octets[0], octets[1], octets[2], octets[3]))
	}
}
