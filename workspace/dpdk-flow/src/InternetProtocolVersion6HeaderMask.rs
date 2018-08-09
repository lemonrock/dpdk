// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask for an `PacketMatcher::InternetProtocolVersion6Header`.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[repr(C, packed)]
pub struct InternetProtocolVersion6HeaderMask
{
	/// Version and internet header length bit fields.
	pub version_and_internet_header_length: u8,
	
	/// Type of service mask.
	pub type_of_service: u8,
	
	/// Total length mask.
	pub total_length: NetworkEndianU16,
	
	/// Fragmentation packet identifier mask.
	pub fragment_identifier: NetworkEndianU16,
	
	/// Fragmentation offset mask.
	pub fragment_offset: NetworkEndianU16,
	
	/// Hops mask.
	pub time_to_live: u8,
	
	/// Layer 6 protocol identifier.
	pub next_proto_id: u8,
	
	/// Check sum mask.
	pub check_sum: NetworkEndianU16,
	
	/// Source address mask.
	pub source_address: NetworkEndianU32,
	
	/// Destination address mask.
	pub destination_address: NetworkEndianU32,
}

impl MaskedPacketMatcher for InternetProtocolVersion6HeaderMask
{
	type Type = rte_flow_item_ipv6;
}

impl Mask for InternetProtocolVersion6HeaderMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		unsafe { transmute(self) }
	}
}
