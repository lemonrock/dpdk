// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A matcher that matches an Internet Protocol (IP) version 4 packet header.
#[derive(Debug)]
#[repr(transparent)]
pub struct InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	underlying: rte_flow_item_ipv4,
}

impl Clone for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		generic_clone(&self)
	}
}

impl PartialEq for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		generic_equals(&self, &rhs)
	}
}

impl Eq for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
}

impl PartialOrd for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		generic_compare(&self, &rhs)
	}
}

impl Hash for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		generic_hash::<H, _>(self, hasher)
	}
}

impl PacketMatcher for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_IPV4;
	
	const IsMeta: bool = false;
}

impl MaskedPacketMatcher for InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	type Mask = rte_flow_item_ipv4;
	
	#[inline(always)]
	fn default_mask() -> &'static Self::Mask
	{
		unsafe { &rte_flow_item_ipv4_mask }
	}
}

impl InternetProtocolVersion4HeaderMaskedPacketMatcher
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(internet_protocol_version_4_packet_header: InternetProtocolVersion4PacketHeader) -> Self
	{
		Self
		{
			underlying: rte_flow_item_ipv4
			{
				hdr: unsafe { transmute(internet_protocol_version_4_packet_header) }
			}
		}
	}
}
