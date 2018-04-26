// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnicastEthernetAddress(pub MediaAccessControlAddress);

impl Serialize for UnicastEthernetAddress
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		self.0.serialize(serializer)
	}
}

impl Deserialize for UnicastEthernetAddress
{
	fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error>
	{
		let inner = MediaAccessControlAddress::deserialize(deserializer)?;
		if inner.is_not_valid_unicast()
		{
			Err(D::Error::custom("Is not a valid unicast address"))
		}
		else
		{
			Ok(UnicastEthernetAddress(inner))
		}
	}
}
