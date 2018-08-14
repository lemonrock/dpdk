// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A packet buffer pool.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketBufferPool(NonNull<rte_mempool>);

impl Drop for PacketBufferPool
{
	fn drop(&mut self)
	{
		unsafe { rte_mempool_free(self.as_ptr()) }
	}
}

impl PrintInformation for PacketBufferPool
{
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { rte_mempool_dump(stream, self.as_ptr()) }
	}
}

impl From<NonNull<rte_mempool>> for PacketBufferPool
{
	fn from(value: NonNull<rte_mempool>) -> Self
	{
		PacketBufferPool(value)
	}
}

macro_rules! bulk_allocate
{
	($self: ident, $size: expr) =>
	{
		{
			let mut mbufs: [*mut rte_mbuf; $size] = unsafe { uninitialized() };
			let result = unsafe { rust_rte_pktmbuf_alloc_bulk($self.as_ptr(), mbufs.as_mut_ptr(), $size) };
			if likely!(result == 0)
			{
				Some(unsafe { transmute(mbufs) })
			}
			else
			{
				forget(mbufs);
				None
			}
		}
	}
}

impl PacketBufferPool
{
	/// Data room size.
	///
	/// The data room size is the amount of data that can be stored in a mbuf including the headroom (`RTE_PKTMBUF_HEADROOM`).
	#[inline]
	pub fn data_room_size(&self) -> u16
	{
		unsafe { rust_rte_pktmbuf_data_room_size(self.as_ptr()) }
	}
	
	/// Application private size.
	///
	/// The application private size of mbuf is a zone located between the rte_mbuf structure and the data buffer where an application can store data associated to a packet.
	#[inline]
	pub fn application_private_size(&self) -> u16
	{
		unsafe { rust_rte_pktmbuf_priv_size(self.as_ptr()) }
	}
	
	/// Allocate a packet buffer and call `rte_pktmbuf_reset` on it.
	///
	/// * The field `priv_size` is set to `self.application_private_size()`\*.
	/// * The field `buf_len` is set to `self.data_room_size()`\*. It is the size in bytes of memory starting at the virtual memory address `buf_addr` (or the physical memory address `buf_iova`/`buf_physaddr`).
	/// * The field `pool` is set to `self.as_ptr()`\*.
	/// * The field `buf_addr` is set correctly\*.
	/// * The field `buf_iova` / `buf_physaddr` is set correctly\*.
	/// * The field `next` will be null.
	/// * The fields `pkt_len`, `tx_offload`, `vlan_tci`, `vlan_tci_outer`, `ol_flags`, `packet_type` and `data_len` will all be zero (0).
	/// * Since `packet_type` is an union, the fields `l2_type`, `l3_type`, `l4_type`, `tun_type`, `inner_esp_next_proto`, `inner_l2_type`, `inner_l3_type` and `inner_l4_type` will all be zero.
	/// * Since `tx_offload` is an union, the fields `l2_len`, `l3_len`, `l4_len`, `tso_segsz`, `outer_l3_len` and `outer_l2_len` will all be zero (0).
	/// * The field `nb_segs` will be one (1).
	/// * The field `port` will be `MBUF_INVALID_PORT` (`::std::u16::MAX`).
	/// * The field `data_off` will be set to the minimum of `RTE_PKTMBUF_HEADROOM` and `buf_len`.
	///
	/// \* According to documentation for `rte_mbuf_raw_alloc`.
	///
	/// It is not clear what values are set for:-
	/// * `rearm_data`
	/// * `refcnt` / `refcnt_atomic`
	/// * `hash` (and all associated union fields)
	/// * `timestamp`
	/// * `timesync`
	/// * `seqn`
	/// * `shinfo`
	#[inline]
	pub fn allocate(&self) -> Option<NonNull<rte_mbuf>>
	{
		let result = unsafe { rust_rte_pktmbuf_alloc(self.as_ptr()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			let mut packet_buffer = unsafe { NonNull::new_unchecked(result) };
			{
				let packet_buffer = unsafe { packet_buffer.as_mut() };
				packet_buffer.hash = rte_mbuf_4::default();
				packet_buffer.timestamp = 0;
				packet_buffer.timesync = 0;
				packet_buffer.seqn = 0;
				packet_buffer.shinfo = null_mut();
			}
			
			Some(packet_buffer)
		}
	}
	
	/// Bulk allocate 2 packet buffers.
	#[inline(always)]
	pub fn bulk_allocate_2(&self) -> Option<[NonNull<rte_mbuf>; 2]>
	{
		bulk_allocate!(self, 2)
	}
	
	/// Bulk allocate 4 packet buffers.
	#[inline(always)]
	pub fn bulk_allocate_4(&self) -> Option<[NonNull<rte_mbuf>; 4]>
	{
		bulk_allocate!(self, 4)
	}
	
	/// Bulk allocate 8 packet buffers.
	#[inline(always)]
	pub fn bulk_allocate_8(&self) -> Option<[NonNull<rte_mbuf>; 8]>
	{
		bulk_allocate!(self, 8)
	}
	
	/// Bulk allocate 16 packet buffers.
	#[inline(always)]
	pub fn bulk_allocate_16(&self) -> Option<[NonNull<rte_mbuf>; 16]>
	{
		bulk_allocate!(self, 16)
	}
	
	/// Bulk allocate 32 packet buffers.
	#[inline(always)]
	pub fn bulk_allocate_32(&self) -> Option<[NonNull<rte_mbuf>; 32]>
	{
		bulk_allocate!(self, 32)
	}
	
	/// Put.
	#[inline(always)]
	pub fn put(&self, packet_buffer: NonNull<rte_mbuf>)
	{
		unsafe { rust_rte_mempool_put(self.as_ptr(), packet_buffer.as_ptr() as *mut c_void) }
	}
	
	/// As pointer.
	#[inline(always)]
	pub fn as_ptr(&self) -> *mut rte_mempool
	{
		self.0.as_ptr()
	}
}
