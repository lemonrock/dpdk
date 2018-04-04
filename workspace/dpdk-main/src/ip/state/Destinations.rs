// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct Destinations
{
	pub ourHardwareAddresses: HashSet<ether_addr>, // Duplicates data in ipStates
	pub sourceEthernetAddressBlackList: SourceEthernetAddressBlackList,
	pub ipStates: HashMap<VirtualLanKey, IpState>,
}

impl Destinations
{
	#[inline(always)]
	pub fn isSourceEthernetAddressOneOfOurEthernetAddresses(&self, sourceEthernetAddress: *const ether_addr) -> bool
	{
		self.ourHardwareAddresses.contains(unsafe { &*sourceEthernetAddress })
	}
	
	#[inline(always)]
	pub fn isOneOfOurEthernetAddresses(&self, ethernetAddress: *const ether_addr) -> bool
	{
		self.ourHardwareAddresses.contains(unsafe { &*ethernetAddress })
	}
	
	#[inline(always)]
	pub fn isSourceEthernetAddressInvalidOrBlackListed(&self, sourceEthernetAddress: *const ether_addr) -> bool
	{
		self.sourceEthernetAddressBlackList.isSourceEthernetAddressInvalidOrBlackListed(unsafe { &*sourceEthernetAddress })
	}
	
	#[inline(always)]
	pub fn ipState(&mut self, virtualLanKey: VirtualLanKey) -> Option<&mut IpState>
	{
		self.ipStates.get_mut(&virtualLanKey)
	}
}
