// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
struct IpNetworkAddressBlackListConfiguration<IpNetworkAddress>
{
	maximumRulesInBlackList: u32,
	numberOfTable8sToAllocateInBlackList: u32,
	ipNetworksToBlackList: Vec<IpNetworkAddress>,
}

impl<IpNetworkAddress> Default for IpNetworkAddressBlackListConfiguration<IpNetworkAddress>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			maximumRulesInBlackList: 0, // No further entries possible
			numberOfTable8sToAllocateInBlackList: 8,
			ipNetworksToBlackList: Default::default(),
		}
	}
}

impl IpNetworkAddressBlackListConfiguration<IpV4NetworkAddress>
{
	#[inline(always)]
	pub fn create(&self, name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>) -> IpV4AddressBlackList
	{
		IpV4AddressBlackList::new(name, logicalCoreMemorySocket, self.maximumRulesInBlackList, self.numberOfTable8sToAllocateInBlackList, self.ipNetworksToBlackList.as_slice())
	}
}

impl IpNetworkAddressBlackListConfiguration<IpV6NetworkAddress>
{
	#[inline(always)]
	pub fn create(&self, name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>) -> IpV6AddressBlackList
	{
		IpV6AddressBlackList::new(name, logicalCoreMemorySocket, self.maximumRulesInBlackList, self.numberOfTable8sToAllocateInBlackList, self.ipNetworksToBlackList.as_slice())
	}
}
