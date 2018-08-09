// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask for an `PacketMatcher::InternetProtocolVersion4Header`.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion4HeaderMask
{
	/// Type mask.
	pub type_: u8,
	
	/// Code mask.
	pub code: u8,
	
	/// The checksum includes the payload.
	pub checksum: NetworkEndianU16,
	
	/// Rest-of-header.
	pub rest_of_header: NetworkEndianU32,
}

impl MaskedPacketMatcher for InternetControlMessageProtocolVersion4HeaderMask
{
	type Type = rte_flow_item_icmp;
}

impl Mask for InternetControlMessageProtocolVersion4HeaderMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		unsafe { transmute(self) }
	}
}
