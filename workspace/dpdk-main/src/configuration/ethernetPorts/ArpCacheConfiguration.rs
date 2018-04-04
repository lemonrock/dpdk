// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
struct ArpCacheConfiguration
{
	timeToLiveInSeconds: u64,
	maximumCapacity: usize,
	overrides: HashMap<IpV4HostAddress, UnicastEthernetAddress>,
	fallbacks: HashMap<IpV4HostAddress, UnicastEthernetAddress>,
}

impl Default for ArpCacheConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			timeToLiveInSeconds: 60 * 20, // 20 minutes, Linux default
			maximumCapacity: 4096,
			overrides: Default::default(),
			fallbacks: Default::default(),
		}
	}
}

impl ArpCacheConfiguration
{
	pub fn createArpCache(&self) -> ArpCache
	{
		let mut overrides = HashMap::with_capacity(self.overrides.len());
		for (hostAddress, unicastAddress) in self.overrides.iter()
		{
			overrides.insert(*hostAddress, ((*unicastAddress).0).0);
		}
		
		let mut fallbacks = HashMap::with_capacity(self.fallbacks.len());
		for (hostAddress, unicastAddress) in self.fallbacks.iter()
		{
			fallbacks.insert(*hostAddress, ((*unicastAddress).0).0);
		}
		
		ArpCache::new(self.timeToLiveInSeconds, self.maximumCapacity, overrides, fallbacks)
	}
}
