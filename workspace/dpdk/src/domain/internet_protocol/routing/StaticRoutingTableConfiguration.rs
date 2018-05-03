// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Configuration used to build a static routing table.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct StaticRoutingTableConfiguration<NetworkAddress: InternetProtocolNetworkAddress>
{
	/// These are the static routes.
	pub static_routes: HashMap<NetworkAddress, Route<NetworkAddress::HostAddress>>,
	
	/// This is used if no other routes match, or if essential information can't be found for a route.
	pub default_route_to_next_hop: EthernetDestination,
}

impl<NetworkAddress: InternetProtocolNetworkAddress> StaticRoutingTableConfiguration<NetworkAddress>
{
	/// Next hop Ethernet (Layer 2) information.
	#[inline(always)]
	pub fn configure(&self) -> StaticRoutingTable<NetworkAddress>
	{
		StaticRoutingTable
		{
			longest_prefix_match:
			{
				let mut look_up_table = IpLookupTable::with_capacity(self.static_routes.len());
				
				for (network_address, static_route) in self.static_routes.iter()
				{
					look_up_table.insert(network_address.network().to_rust_address(), network_address.mask_bits_as_depth_u32(), static_route.clone());
				}
			},
			default_route_to_next_hop: self.default_route_to_next_hop,
		}
	}
}
