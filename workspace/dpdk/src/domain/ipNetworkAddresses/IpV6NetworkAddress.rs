// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct IpV6NetworkAddress
{
	#[serde(serialize_with = "IpV6NetworkAddress::serde_serialize_network", deserialize_with = "IpV6NetworkAddress::serde_deserialize_network")] pub network: IpV6HostAddress,
	pub maskBits: IpV6MaskBits,
}

impl IpNetworkAddress for IpV6NetworkAddress
{
	type IpHostAddress = IpV6HostAddress;
	
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

impl IpV6NetworkAddress
{
	fn serde_serialize_network<S: Serializer>(value: &IpV6HostAddress, serializer: S) -> Result<S::Ok, S::Error>
	{
		Ipv6Addr::from(*value).serialize(serializer)
	}
	
	fn serde_deserialize_network<D: Deserializer>(deserializer: D) -> Result<IpV6HostAddress, D::Error>
	{
		let ipv6Addr = Ipv6Addr::deserialize(deserializer)?;
		Ok(ipv6Addr.octets())
	}
}
