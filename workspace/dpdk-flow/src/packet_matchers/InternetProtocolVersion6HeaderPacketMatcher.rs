// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A matcher that matches an Internet Protocol (IP) version 6 packet header.
///
/// There is also another matcher rte_flow_item_ipv6_ext for matching the presence of extension headers.
#[derive(Debug)]
#[repr(transparent)]
pub struct InternetProtocolVersion6HeaderPacketMatcher
{
	underlying: rte_flow_item_ipv6,
}

impl Clone for InternetProtocolVersion6HeaderPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		generic_clone(&self)
	}
}

impl PartialEq for InternetProtocolVersion6HeaderPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		generic_equals(&self, &rhs)
	}
}

impl Eq for InternetProtocolVersion6HeaderPacketMatcher
{
}

impl PartialOrd for InternetProtocolVersion6HeaderPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for InternetProtocolVersion6HeaderPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		generic_compare(&self, &rhs)
	}
}

impl Hash for InternetProtocolVersion6HeaderPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		generic_hash::<H, _>(self, hasher)
	}
}

impl PacketMatcher for InternetProtocolVersion6HeaderPacketMatcher
{
	type DpdkType = rte_flow_item_ipv6;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_IPV4;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_ipv6_mask }
	}
}

impl InternetProtocolVersion6HeaderPacketMatcher
{
	/// A `header.source_address` of 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF matches all Internet Protocol (IP) version 6 source addresses.
	/// A `header.destination_address` of 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF matches all Internet Protocol (IP) version 6 destination addresses.
	#[inline(always)]
	pub fn new(internet_protocol_version_6_packet_header: InternetProtocolVersion6PacketHeader) -> Self
	{
		Self
		{
			underlying: rte_flow_item_ipv6
			{
				hdr: unsafe { transmute(internet_protocol_version_6_packet_header) }
			}
		}
	}
}
