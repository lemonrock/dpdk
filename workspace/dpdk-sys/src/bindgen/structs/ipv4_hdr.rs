// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct ipv4_hdr
{
	pub version_ihl: u8,
	pub type_of_service: u8,
	pub total_length: u16,
	pub packet_id: u16,
	pub fragment_offset: u16,
	pub time_to_live: u8,
	pub next_proto_id: u8,
	pub hdr_checksum: u16,
	pub src_addr: u32,
	pub dst_addr: u32,
}

impl Default for ipv4_hdr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ipv4_hdr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ipv4_hdr {{  }}")
	}
}
