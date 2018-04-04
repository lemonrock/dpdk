// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressWithListOfOpenLocalLayer4Ports<A>
{
	ipAddress: A,
	whitelistOfPorts: HashSet<Layer4Port>,
}

impl<A> AddressWithListOfOpenLocalLayer4Ports<A>
{
	#[inline(always)]
	pub fn new(ipAddress: A, whitelistOfPorts: HashSet<Layer4Port>) -> Self
	{
		AddressWithListOfOpenLocalLayer4Ports
		{
			ipAddress: ipAddress,
			whitelistOfPorts: whitelistOfPorts,
		}
	}
	
	#[inline(always)]
	pub fn blockedPortsForTldk(&self) -> TldkBlockedPortsList
	{
		const MinimumPortInclusive: usize = 1;
		const MaximumPortExclusive: usize = 65536;
		
		let mut blockedPorts = Vec::with_capacity(MaximumPortExclusive - self.whitelistOfPorts.len());
		// We include Port 0 for TLDK, even though it's not a valid TCP or UDP port
		blockedPorts.push(0);
		
		for port in MinimumPortInclusive..MaximumPortExclusive
		{
			let port = port as u16;
			if !self.whitelistOfPorts.contains(&Layer4Port::convertFromTcpOrUdpOrSctpPortValueInLayer4Header(port).unwrap())
			{
				blockedPorts.push(port);
			}
		}
		
		blockedPorts.sort();
		
		return blockedPorts
	}
}

impl AddressWithListOfOpenLocalLayer4Ports<Ipv4Addr>
{
	#[inline(always)]
	pub fn in_addr(&self) -> in_addr
	{
		IpV4HostAddress::from_Ipv4Addr_to_in_addr(&self.ipAddress)
	}
}

impl AddressWithListOfOpenLocalLayer4Ports<Ipv6Addr>
{
	#[inline(always)]
	pub fn in6_addr(&self) -> in6_addr
	{
		IpV6HostAddress::from_Ipv6Addr_to_in6_addr(&self.ipAddress)
	}
}
