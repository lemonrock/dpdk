// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct IpV4AddressBlackList
{
	longestPrefixMatchTable: Arc<RwLock<IpV4LongestPrefixMatchTable>>,
}

impl IpAddressBlackList for IpV4AddressBlackList
{
	type IpHostAddress = IpV4HostAddress;
	
	type IpNetworkAddress = IpV4NetworkAddress;
	
	type LongestPrefixMatchTable = IpV4LongestPrefixMatchTable;
	
	const NamePrefix: &'static str = "IPv4-BL";
	
	#[inline(always)]
	fn _new(longestPrefixMatchTable: Self::LongestPrefixMatchTable) -> Self
	{
		Self
		{
			longestPrefixMatchTable: Arc::new(RwLock::new(longestPrefixMatchTable))
		}
	}
	
	#[inline(always)]
	fn _longestPrefixMatchTableConst(&self) -> RwLockReadGuard<Self::LongestPrefixMatchTable>
	{
		self.longestPrefixMatchTable.read().unwrap()
	}
	
	#[inline(always)]
	fn _longestPrefixMatchTableMut(&mut self) -> RwLockWriteGuard<Self::LongestPrefixMatchTable>
	{
		self.longestPrefixMatchTable.write().unwrap()
	}
}
