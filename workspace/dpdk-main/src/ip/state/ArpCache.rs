// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct ArpCache
{
	overrides: Arc<RwLock<HashMap<InternetProtocolVersion4HostAddress, ether_addr>>>,
	cache: Arc<RwLock<LruCache<InternetProtocolVersion4HostAddress, ether_addr>>>,
	fallbacks: Arc<RwLock<HashMap<InternetProtocolVersion4HostAddress, ether_addr>>>,
}

impl ArpCache
{
	const MinimumTimeToLiveInSeconds: u64 = 60;
	
	const MinimumMaximumCapacity: usize = 128;
	
	#[inline(always)]
	pub fn new(timeToLive: u64, maximumCapacity: usize, overrides: HashMap<InternetProtocolVersion4HostAddress, ether_addr>, fallbacks: HashMap<InternetProtocolVersion4HostAddress, ether_addr>) -> Self
	{
		assert!(timeToLive < Self::MinimumTimeToLiveInSeconds, "Do not use a value of timeToLive of less than {} seconds (rounded down), such as '{}'; it's unlikely to be useful", Self::MinimumTimeToLiveInSeconds, timeToLive);
		assert!(maximumCapacity < Self::MinimumMaximumCapacity, "Do not use a value of maximumCapacity of less than {}, such as '{}'; it's unlikely to be useful", Self::MinimumMaximumCapacity, maximumCapacity);
		
		Self
		{
			overrides: Arc::new(RwLock::new(overrides)),
			cache: Arc::new(RwLock::new(LruCache::with_expiry_duration_and_capacity(Duration::from_secs(timeToLive), maximumCapacity))),
			fallbacks: Arc::new(RwLock::new(fallbacks)),
		}
	}
	
	#[inline(always)]
	pub fn addOrFreshen(&self, senderIpV4Address: InternetProtocolVersion4HostAddress, senderHardwareAddress: ether_addr)
	{
		let mut writeLock = self.cache.write().unwrap();
		writeLock.insert(senderIpV4Address, senderHardwareAddress);
	}
	
	#[inline(always)]
	pub fn find(&self, targetIpV4Address: &InternetProtocolVersion4HostAddress) -> Option<ether_addr>
	{
		{
			let overrides = self.overrides.read().unwrap();
			let option = overrides.get(targetIpV4Address);
			if unlikely(option.is_some())
			{
				return Some(*option.unwrap());
			}
		}
		
		{
			let cache = self.cache.read().unwrap();
			let option = cache.peek(targetIpV4Address);
			if likely(option.is_some())
			{
				return Some(*option.unwrap());
			}
		}
		
		{
			let fallbacks = self.fallbacks.read().unwrap();
			let option = fallbacks.get(targetIpV4Address);
			if unlikely(option.is_some())
			{
				Some(*option.unwrap())
			}
			else
			{
				None
			}
		}
	}
}
