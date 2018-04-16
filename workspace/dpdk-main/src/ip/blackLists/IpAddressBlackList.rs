// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait IpAddressBlackList
{
	type InternetProtocolHostAddress;

	type InternetProtocolNetworkAddress: InternetProtocolNetworkAddress<InternetProtocolHostAddress=Self::InternetProtocolHostAddress>;

	type LongestPrefixMatchTable: LongestPrefixMatchTable<InternetProtocolHostAddress=Self::InternetProtocolHostAddress, InternetProtocolNetworkAddress=Self::InternetProtocolNetworkAddress>;

	const NamePrefix: &'static str;

	#[inline(always)]
	fn _new(longestPrefixMatchTable: Self::LongestPrefixMatchTable) -> Self;

	#[inline(always)]
	fn _longestPrefixMatchTableConst(&self) -> RwLockReadGuard<Self::LongestPrefixMatchTable>;

	#[inline(always)]
	fn _longestPrefixMatchTableMut(&mut self) -> RwLockWriteGuard<Self::LongestPrefixMatchTable>;

	const OurNextHop: u32 = 0xFFFFFFFF;

	#[inline(always)]
	fn new(name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>, maximumRules: u32, numberOfTable8sToAllocate: u32, networkAddresses: &[Self::InternetProtocolNetworkAddress]) -> Self
	where Self: Sized
	{
		let name = name.toName(Self::NamePrefix);

		let maximumRules = max(maximumRules, networkAddresses.len() as u32);

		let table = LongestPrefixMatchTable::new(&name, maximumRules, numberOfTable8sToAllocate, logicalCoreMemorySocket).expect("Could not allocate");
		let mut this = Self::_new(table);

		for networkAddress in networkAddresses.iter()
		{
			this.addNetworkToBlackList(networkAddress);
		}

		this
	}

	#[inline(always)]
	fn isIpAddressBlackListed(&self, hostAddress: &Self::InternetProtocolHostAddress) -> bool
	{
		if let Some(nextHop) = self._longestPrefixMatchTableConst().look_up(hostAddress)
		{
			nextHop == Self::OurNextHop
		}
		else
		{
			false
		}
	}

	#[inline(always)]
	fn addNetworkToBlackList(&mut self, networkAddress: &Self::InternetProtocolNetworkAddress) -> bool
	{
		self._longestPrefixMatchTableMut().addRule(networkAddress, Self::OurNextHop)
	}
}
