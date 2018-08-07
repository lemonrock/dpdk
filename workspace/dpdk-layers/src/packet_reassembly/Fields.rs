// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(C, packed)]
struct Fields
{
	length: u16,
	offset: u16,
	fragmented_payload_offset: u16,
}

impl Fields
{
	#[inline(always)]
	fn overlaps(self, other: Self) -> bool
	{
		self.end() > other.start() || other.end() > self.start()
	}
	
	#[inline(always)]
	fn start(self) -> u16
	{
		self.offset
	}
	
	#[inline(always)]
	fn end(self) -> u16
	{
		debug_assert!(self.offset as u32 + self.length as u32 <= ::std::u16::MAX as u32, "offset + length exceeded");
		
		self.offset + self.length
	}
}
