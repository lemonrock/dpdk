// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A static routing table.
pub struct StaticRoutingTable<'deserialize, NetworkAddress: InternetProtocolNetworkAddress<'deserialize>>
{
	longest_prefix_match: IpLookupTable<<<NetworkAddress as InternetProtocolNetworkAddress<'deserialize>>::HostAddress as InternetProtocolHostAddress<'deserialize>>::RustAddress, Route<'deserialize, NetworkAddress::HostAddress>>,
	default_route_to_next_hop: EthernetDestination,
}

impl<'deserialize, NetworkAddress: InternetProtocolNetworkAddress<'deserialize>> StaticRoutingTable<'deserialize, NetworkAddress>
{
	/// Finds the information necessary to send to this address.
	#[inline(always)]
	pub fn route_to_next_hop(&self, destination_internet_protocol_host_address: &NetworkAddress) -> EthernetDestination
	{
		match self.longest_prefix_match.longest_match(destination_internet_protocol_host_address)
		{
			None => self.default_route_to_next_hop,
			
			Some(route) =>
			{
				let next_hop_internet_protocol_host_address = route.next_hop_internet_protocol_host_address(destination_internet_protocol_host_address);
				match route.next_hop_media_access_control_address(next_hop_internet_protocol_host_address)
				{
					Err(()) => self.default_route_to_next_hop,
					
					Ok(media_access_control_address) =>
					{
						EthernetDestination
						{
							media_access_control_address,
							ethernet_frame_length: route.next_hop_ethernet_frame_length(),
						}
					}
				}
			}
		}
	}
}
