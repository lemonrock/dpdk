// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask for an `Pattern::InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisement`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementMask
{
	/// Header check sum mask.
	pub header_check_sum: NetworkEndianU16,
	
	/// Target address mask.
	pub target_address: NetworkEndianU128,
	
	/// Flags.
	pub flags: InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementFlags,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_icmp6_nd_na,
}

custom_deserialize!
{
	InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementMask,
	0 => header_check_sum,
	1 => target_address,
	2 => flags,
}

impl MaskedPattern for InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementMask
{
	type Type = rte_flow_item_icmp6_nd_na;
}

impl Mask for InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPattern>::Type
	{
		&self.cached
	}
}

impl InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementMask
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(header_check_sum: NetworkEndianU16, target_address: NetworkEndianU128, flags: InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementFlags) -> Self
	{
		Self
		{
			header_check_sum,
			target_address,
			flags,
			cached: rte_flow_item_icmp6_nd_na
			{
				type_: InternetControlMessageProtocolVersion6Type::NeighborSolicitation.into(),
				code: 0,
				checksum: header_check_sum.to_network_endian(),
				rso_reserved: flags.bits() as u32,
				target_addr: target_address.to_bytes(),
			}
		}
	}
}
