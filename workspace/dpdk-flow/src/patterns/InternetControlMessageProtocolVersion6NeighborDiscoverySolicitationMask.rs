// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification for an `Pattern::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	/// Header check sum mask.
	pub header_check_sum: NetworkEndianU16,
	
	/// Target address mask.
	pub target_address: NetworkEndianU128,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_icmp6_nd_ns,
}

custom_deserialize!
{
	InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask,
	0 => header_check_sum,
	1 => target_address,
}

impl Clone for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			header_check_sum: self.header_check_sum,
			target_address: self.target_address,
			cached: bitwise_clone!(self, rte_flow_item_icmp6_nd_ns),
		}
	}
}

impl PartialOrd for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.header_check_sum.cmp(&rhs.header_check_sum).then_with(|| self.target_address.cmp(&rhs.target_address))
	}
}

impl PartialEq for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.header_check_sum == rhs.header_check_sum && self.target_address == rhs.target_address
	}
}

impl Eq for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
}

impl Hash for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.header_check_sum.hash(hasher);
		self.target_address.hash(hasher)
	}
}

impl MaskedPattern for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	type Type = rte_flow_item_icmp6_nd_ns;
}

impl Mask for InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPattern>::Type
	{
		&self.cached
	}
}

impl InternetControlMessageProtocolVersion6NeighborDiscoverySolicitationMask
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(header_check_sum: NetworkEndianU16, target_address: NetworkEndianU128) -> Self
	{
		Self
		{
			header_check_sum,
			target_address,
			cached: rte_flow_item_icmp6_nd_ns
			{
				type_: InternetControlMessageProtocolVersion6Type::NeighborSolicitation.into(),
				code: 0,
				checksum: header_check_sum.to_network_endian(),
				reserved: 0,
				target_addr: target_address.to_bytes(),
			}
		}
	}
}
