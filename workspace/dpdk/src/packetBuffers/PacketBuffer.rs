// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketBuffer(*mut rte_mbuf);

impl PacketBuffer
{
	#[inline(always)]
	pub fn dumpToStandardError(&self, length: u32)
	{
		unsafe { rte_pktmbuf_dump(stderr as *mut FILE, self.0, length) };
	}
}

impl Drop for PacketBuffer
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rust_rte_pktmbuf_free(self.0) }
	}
}

impl PacketBuffer
{
	#[inline(always)]
	pub fn clone(&self, packetBufferPool: PacketBufferPool) -> Option<PacketBuffer>
	{
		let result = unsafe { rust_rte_pktmbuf_clone(self.0, packetBufferPool.0) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(PacketBuffer(result))
		}
	}

	#[inline(always)]
	pub fn reset(&self)
	{
		unsafe { rust_rte_pktmbuf_reset(self.0) }
	}

	#[inline(always)]
	pub fn attach(&self, attach: PacketBuffer)
	{
		unsafe { rust_rte_pktmbuf_attach(self.0, attach.0) }
	}

	#[inline(always)]
	pub fn detach(&self)
	{
		unsafe { rust_rte_pktmbuf_detach(self.0) }
	}

	#[inline(always)]
	pub fn freeSegment(&self)
	{
		unsafe { rust_rte_pktmbuf_free_seg(self.0) }
	}

	#[inline(always)]
	pub fn updateReferenceCount(&self, delta: i16)
	{
		unsafe { rust_rte_pktmbuf_refcnt_update(self.0, delta) }
	}

	#[inline(always)]
	pub fn headRoom(&self) -> u16
	{
		unsafe { rust_rte_pktmbuf_headroom(self.0) }
	}

	#[inline(always)]
	pub fn tailRoom(&self) -> u16
	{
		unsafe { rust_rte_pktmbuf_tailroom(self.0) }
	}

	#[inline(always)]
	pub fn lastSegment(&self) -> Option<PacketBuffer>
	{
		let result = unsafe { rust_rte_pktmbuf_lastseg(self.0) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(PacketBuffer(result))
		}
	}

	#[inline(always)]
	pub fn prepend(&self, length: u16) -> *mut c_char
	{
		unsafe { rust_rte_pktmbuf_prepend(self.0, length) }
	}

	#[inline(always)]
	pub fn append(&self, length: u16) -> *mut c_char
	{
		unsafe { rust_rte_pktmbuf_append(self.0, length) }
	}

	#[inline(always)]
	pub fn adjust(&self, length: u16) -> *mut c_char
	{
		unsafe { rust_rte_pktmbuf_adj(self.0, length) }
	}

	#[inline(always)]
	pub fn trim(&self, length: u16) -> bool
	{
		let result = unsafe { rust_rte_pktmbuf_trim(self.0, length) };
		isTrue(result)
	}

	#[inline(always)]
	pub fn isContiguous(&self) -> bool
	{
		let result = unsafe { rust_rte_pktmbuf_is_contiguous(self.0) };
		isTrue(result)
	}

	#[inline(always)]
	fn chain(head: &PacketBuffer, tail: &PacketBuffer) -> bool
	{
		let result = unsafe { rust_rte_pktmbuf_chain(head.0, tail.0) };
		if likely(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				NegativeE::EOVERFLOW => false,

				_ => panic!("Unexpected error code '{}' from rust_rte_rte_pktmbuf_chain()", result),
			}
		}
	}

	#[inline(always)]
	pub fn chainTail(&self, tail: &PacketBuffer) -> bool
	{
		Self::chain(self, tail)
	}

	#[inline(always)]
	pub fn chainHead(&self, head: &PacketBuffer) -> bool
	{
		Self::chain(head, self)
	}

	#[inline(always)]
	pub fn mtophys_mtod_offset(&self, offset: usize) -> *mut c_void
	{
		unsafe { rust_rte_pktmbuf_mtophys_mtod_offset(self.0, offset) }
	}

	#[inline(always)]
	pub fn mtophys_mtod(&self) -> *mut c_void
	{
		unsafe { rust_rte_pktmbuf_mtophys_mtod(self.0) }
	}

	#[inline(always)]
	pub fn mtophys_offset(&self, offset: usize) -> phys_addr_t
	{
		unsafe { rust_rte_pktmbuf_mtophys_offset(self.0, offset) }
	}

	#[inline(always)]
	pub fn mtophys(&self) -> phys_addr_t
	{
		unsafe { rust_rte_pktmbuf_mtophys(self.0) }
	}

	#[inline(always)]
	pub fn length(&self) -> u32
	{
		unsafe { rust_rte_pktmbuf_pkt_len(self.0) }
	}

	#[inline(always)]
	pub fn dataLength(&self) -> u16
	{
		unsafe { rust_rte_pktmbuf_data_len(self.0) }
	}
}
