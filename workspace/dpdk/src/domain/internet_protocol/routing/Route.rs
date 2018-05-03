// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a route.
///
/// Contains:-
///
/// * the location of a next-hop router, `next_hop_router`, if any; if not known, then one can assume this route is point-to-point.
/// * the media access control address of the destination (destination ethernet address); if not known, then one can deduce this using the IP address to create it use a custom scheme akin to DECnet.
/// * the ethernet frame length to use for this route. If not known, then this can be computed either from the Media Access Control Address of the destination OR it can be the default.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Route<HostAddress: InternetProtocolHostAddress>
{
	/// The host address of the next hop / router, if any.
	///
	/// If not specified, then it is assumed the next hop is directly accessible.
	pub router: Option<HostAddress>,
	
	/// The media access control address of the router of this route.
	///
	/// If not specified, then it is derived from the next hop (or router's) internet protocol address.
	///
	/// If this fails to produce a valid address (and, for version 4, a private address), then default information will be used from the StaticRoutingTable.
	///
	/// In effect, the fallback strategy is to send packets to a known default router.
	pub media_access_control_address: Option<MediaAccessControlAddress>,
	
	/// The ethernet frame length of the destination.
	///
	/// Defaults to `MaximumIncludingCyclicRedundancyCheckWithoutJumboFrames`, 1518 bytes.
	pub ethernet_frame_length: EthernetFrameLength,
}

impl<HostAddress: InternetProtocolHostAddress> Route<HostAddress>
{
	/// Next hop internet protocol host address.
	#[inline(always)]
	pub fn next_hop_internet_protocol_host_address(&self, destination_internet_protocol_host_address: &HostAddress) -> &HostAddress
	{
		match self.router
		{
			None => destination_internet_protocol_host_address,
			Some(ref router) => router,
		}
	}
	
	/// If there is no static media access control address and `next_hop_internet_protocol_host_address` address is not a suitable address, then an `Err(())` is returned.
	///
	/// In the event of an error, one possible fallback would be to then send the packets to the media access control address of the default next_hop_internet_protocol_host_address for the network we are within.
	#[inline(always)]
	pub fn next_hop_media_access_control_address(&self, next_hop_internet_protocol_host_address: &HostAddress) -> Result<MediaAccessControlAddress, ()>
	{
		match self.media_access_control_address
		{
			Some(media_access_control_address) => media_access_control_address,
			None => next_hop_internet_protocol_host_address.to_media_access_control_address(),
		}
	}
	
	/// Ethernet frame length for this route.
	#[inline(always)]
	pub fn next_hop_ethernet_frame_length(&self) -> EthernetFrameLength
	{
		self.ethernet_frame_length
	}
}
