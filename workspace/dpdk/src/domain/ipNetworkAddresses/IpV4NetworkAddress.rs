// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct IpV4NetworkAddress
{
	#[serde(serialize_with = "IpV4NetworkAddress::serde_serialize_network", deserialize_with = "IpV4NetworkAddress::serde_deserialize_network")] pub network: InternetProtocolVersion4HostAddress,
	pub maskBits: IpV4MaskBits,
}

impl IpNetworkAddress for IpV4NetworkAddress
{
	type IpHostAddress = InternetProtocolVersion4HostAddress;
	
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
		network: InternetProtocolVersion4HostAddress([127, 0, 0, 0]),
		maskBits: IpV4MaskBits::_8,
	};
	
	pub const Multicast: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([224, 0, 0, 0]),
		maskBits: IpV4MaskBits::_4,
	};
	
	// RFC 5737
	pub const TestNet1: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([192, 0, 2, 0]),
		maskBits: IpV4MaskBits::_24,
	};
	
	// RFC 5737
	pub const TestNet2: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([198, 51, 100, 0]),
		maskBits: IpV4MaskBits::_24,
	};
	
	// RFC 5737
	pub const TestNet3: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([203, 0, 113, 0]),
		maskBits: IpV4MaskBits::_24,
	};
	
	// RFC 1918
	pub const Private1: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([10, 0, 0, 0]),
		maskBits: IpV4MaskBits::_8,
	};
	
	// RFC 1918
	pub const Private2: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([172, 16, 0, 0]),
		maskBits: IpV4MaskBits::_12,
	};
	
	// RFC 1918
	pub const Private3: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([192, 168, 0, 0]),
		maskBits: IpV4MaskBits::_16,
	};
	
	// RFC 3927
	pub const LinkLocal: IpV4NetworkAddress = IpV4NetworkAddress
	{
		network: InternetProtocolVersion4HostAddress([169, 254, 0, 0]),
		maskBits: IpV4MaskBits::_16,
	};
	
	#[inline(always)]
	pub fn contains(&self, ipV4HostAddress: InternetProtocolVersion4HostAddress) -> bool
	{
		ipV4HostAddress & self.maskBits.asMask() == self.network
	}
	
	fn serde_serialize_network<S: Serializer>(value: &InternetProtocolVersion4HostAddress, serializer: S) -> Result<S::Ok, S::Error>
	{
		Ipv4Addr::from(*value).serialize(serializer)
	}
	
	fn serde_deserialize_network<D: Deserializer>(deserializer: D) -> Result<InternetProtocolVersion4HostAddress, D::Error>
	{
		let ipv4_addr = Ipv4Addr::deserialize(deserializer)?;
		Ok(InternetProtocolVersion4HostAddress::from_ipv4_addr(&ipv4_addr))
	}
}
