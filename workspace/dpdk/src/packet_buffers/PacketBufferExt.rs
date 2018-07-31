// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An extension trait which makes a `NonNull<rte_mbuf>` appear as a regular object, `PacketBuffer`.
pub trait PacketBufferExt: PrintInformation
{
	/// Returns the IO address that points to the start of the data in the packet.
	///
	/// Implements `rte_pktmbuf_iova`.
	#[inline(always)]
	fn io_virtual_address(self) -> rte_iova_t
	{
		self.io_virtual_address_offset(0)
	}
	
	/// Returns the IO address that points to an offset of the data in the packet.
	///
	/// Implements `rte_pktmbuf_iova_offset`.
	#[inline(always)]
	fn io_virtual_address_offset(self, offset: u64) -> rte_iova_t
	{
		let packet = self.reference();
		((*packet._1.buf_iova.as_ref()) + (self.segment_buffer_reserved_head_room() as u64) + offset) as rte_iova_t
	}
	
	/// Returns an error on overflow.
	#[inline(always)]
	fn chain_append_tail(&self, tail: PacketBuffer) -> Result<(), ()>
	{
		Self::chain_together(self, tail)
	}
	
	/// Returns an error on overflow.
	#[inline(always)]
	fn chain_prepend_head(&self, head: PacketBuffer) -> Result<(), ()>
	{
		Self::chain_together(head, self)
	}
	
	/// Chain together.
	#[inline(always)]
	fn chain_together(head: PacketBuffer, tail: PacketBuffer) -> Result<(), ()>
	{
		let result = unsafe { rust_rte_pktmbuf_chain(head.as_ptr(), tail.as_ptr()) };
		if likely!(result == 0)
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
	
	/// User data as a (possibly null) pointer.
	#[inline(always)]
	fn user_data_as_pointer<T>(self) -> *mut T
	{
		self.reference()._4.userdata as *mut T
	}
	
	/// User data as 8 native endian bytes.
	///
	/// Used for instance by the `rte_security` library.
	#[inline(always)]
	fn user_data_raw(self) -> u64
	{
		self.reference()._4.udata64
	}
	
	/// Parent packet buffer pool that allocated this packet.
	#[inline(always)]
	fn packet_buffer_pool_packet_allocated_from(self) -> PacketBufferPool
	{
		unsafe { NonNull::new_unchecked(self.as_ref().pool) }
	}
	
	/// Optimized routine that only works on direct, contiguous packets with a reference count of 1.
	#[inline(always)]
	fn free_direct_contiguous_packet(self)
	{
		self.raw_free()
	}
	
	/// Put packet back into its original packet buffer pool.
	///
	/// The caller must ensure that the mbuf is direct and properly reinitialized (`refcnt=1`, `next=NULL`, `nb_segs=1`), as done by `self.pre_free_segment()`.
	///
	/// This function should be used with care, when optimization is required.
	///
	/// For standard needs, prefer `self.free()` or `self.free_segment()`.
	#[inline(always)]
	fn raw_free(self)
	{
		debug_assert_ne!(self.is_indirect_attached_packet_buffer(), "This is an indirect packet");
		self.debug_assert_is_contiguous();
		debug_assert_eq!(self.reference_count(), 1, "Has a reference count which is not 1");
		
		self.packet_buffer_pool_packet_allocated_from().put(self.as_ptr())
	}
	
	/// Decreases reference counter and unlinks a mbuf segment.
	///
	/// This function does the same than a free, except that it does not return the segment to its packet buffer pool.
	///
	/// It decreases the reference counter, and if it reaches 0, it is detached from its parent for an indirect mbuf.
	///
	/// Returns Some(self) if is the last reference, which can be recycled of freed. Otherwise returns None if the reference count is not zero.
	#[inline(always)]
	fn pre_free_segment(self) -> Option<PacketBuffer>
	{
		let result = unsafe { rust_rte_pktmbuf_prefree_seg(self.as_ptr()) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(result) })
		}
	}
	
	/// Free a segment of a packet into its original packet buffer pool.
	///
	/// Does so without parsing other segments in the case of chained buffers.
	#[inline(always)]
	fn free_segment(self)
	{
		unsafe { rust_rte_pktmbuf_free_seg(self.as_ptr()) }
	}
	
	/// Next segment.
	#[inline(always)]
	fn next_segment(self) -> Option<NonNull<PacketBuffer>>
	{
		let next = self.reference().next;
		if next.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(next) })
		}
	}
	
	/// Last segment.
	#[inline(always)]
	fn last_segment(self) -> Option<NonNull<PacketBuffer>>
	{
		let result = unsafe { rust_rte_pktmbuf_lastseg(self.as_ptr()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(result) })
		}
	}
	
	
	/// This function moves the data into the first segment if there is enough tail room.
	///
	/// In effect, compaction to try to make a Packet Buffer contiguous.
	///
	/// The subsequent segments are unchained and freed.
	#[inline(always)]
	fn linearize(self)
	{
		unsafe { rust_rte_pktmbuf_linearize(self.as_ptr()) }
	}
	
	/// Current reference count.
	#[inline(always)]
	fn reference_count(self) -> u16
	{
		self.reference().refcnt
	}
	
	/// Adjust reference count by delta for all segments.
	#[inline(always)]
	fn adjust_reference_count_for_all_segments(self, delta: i16)
	{
		let mut m = self.as_ptr();
		while
		{
			unsafe { rust_rte_mbuf_refcnt_update(m, delta) };
			m = unsafe { & * m }.next;
			m.is_not_null()
		}
		{
		}
	}
	
	/// Packet (*not TCP*) sequence number.
	///
	/// Used for re-ordering out-of-order packets, typically when packets are being received by multiple threads.
	/// In this case, the sequence number can be a global atomically incremented counter.
	///
	/// See `ReorderBuffer`.
	#[inline(always)]
	fn sequence_number(self) -> u32
	{
		self.reference().seqn
	}
	
	/// Packet (*not TCP*) sequence number.
	///
	/// Used for re-ordering out-of-order packets, typically when packets are being received by multiple threads.
	/// In this case, the sequence number can be a global atomically incremented counter.
	///
	/// See `ReorderBuffer`.
	#[inline(always)]
	fn set_sequence_number(self, sequence_number: u32)
	{
		self.mutable_reference().seqn = sequence_number
	}
	
	/// Clone.
	///
	/// Allocates the clone from `packet_buffer_pool`.
	#[inline(always)]
	fn clone(&self, packet_buffer_pool: PacketBufferPool) -> Result<NonNull<PacketBuffer>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_clone(self.as_ptr(), packet_buffer_pool.as_ptr()) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result) })
		}
	}
	
	/// Reset the fields of a packet to their default values; allows re-use.
	///
	/// The packet must have only one segment.
	#[inline(always)]
	fn reset(self)
	{
		unsafe { rust_rte_pktmbuf_reset(self.as_ptr()) }
	}
	
	/// Resets head room length to the minimum of `Self::HeadRoom` and `self.segment_buffer_length()`.
	///
	/// The packet must have only one segment.
	///
	/// Does not move any data at all, so all data (eg headers) will be invalid after this.
	///
	/// Returns the new segment buffer reserved head room.
	#[inline(always)]
	fn reset_segment_buffer_reserved_head_room(self) -> u16
	{
		let head_room = min(Self::HeadRoom, self.segment_buffer_length());
		self.mutable_reference().data_off = head_room;
		
		head_room
	}
	
	/// Attach packet `attach_as_indirect` to this one.
	///
	/// After attachment we refer to the packet buffer we attached as 'indirect', while we refer to the mbuf we attached to as 'direct'.
	///
	/// The direct mbuf's reference counter is incremented.
	///
	/// Currently the following are not supported:-
	/// * `attach_as_indirect` is already indirectly attached.
	/// * `attach_as_indirect` is used by someone else (its reference counter is greater then 1).
	#[inline(always)]
	fn attach(self, attach_as_indirect: PacketBuffer)
	{
		unsafe { rust_rte_pktmbuf_attach(attach_as_indirect.as_ptr(), self.as_ptr()) }
	}
	
	/// Detach an indirect packet.
	///
	/// * restore original mbuf address and length values.
	/// * reset pktmbuf data and data_len to their default values.
	/// * decrement the direct mbuf's reference counter.
	///
	/// When the reference counter becomes 0, the direct mbuf is freed.
	///
	/// All other fields of the given packet will be left intact.
	#[inline(always)]
	fn detach_indirect_packet(self)
	{
		unsafe { rust_rte_pktmbuf_detach(self.as_ptr()) }
	}
	
	/// Append `length` bytes to the packet and return a pointer to the start address of the added data.
	///
	/// If there is not enough head room in the first segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	fn prepend(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_prepend(self.as_ptr(), length) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result as *mut u8) })
		}
	}
	
	/// Append `length` bytes to the packet and return a pointer to the start address of the added data.
	///
	/// If the `length` is greater than the length of the last segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	fn append(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_append(self.as_ptr(), length) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result as *mut u8) })
		}
	}
	
	/// Remove `length` bytes at the beginning of a packet and return a pointer to the start address of the new data area.
	///
   /// If the `length` is greater than the length of the first segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	fn remove(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_adj(self.as_ptr(), length) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result as *mut u8) })
		}
	}
	
	/// Remove `length` bytes of data at the end of the packet.
	///
	/// If the `length` is greater than the length of the last segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	fn trim(self, length: u16) -> Result<(), ()>
	{
		let result = unsafe { rust_rte_pktmbuf_trim(self.as_ptr(), length) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if result == -1
		{
			Err(())
		}
		else
		{
			panic!("Unexpected result")
		}
	}
	
	/// Validate that the packet's fields are correctly set for transmit offload.
	#[inline(always)]
	fn validate_transmit_offload(self) -> Result<(), PosixErrorNumber>
	{
		let result = unsafe { rust_rte_validate_tx_offload(self.as_ptr() as *const _) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result < 0)
		{
			Err(-result)
		}
		else
		{
			panic!("Invalid result '{}' from rust_rte_validate_tx_offload()", result)
		}
	}
	
	/// Is the hash a receive side scaling hash?
	#[inline(always)]
	fn has_receive_side_scaling_hash(self) -> bool
	{
		self.has_offload_flag(PKT_RX_RSS_HASH)
	}
	
	/// Receive side scaling hash.
	///
	/// Only valid if `self.has_receive_side_scaling_hash()` is true.
	#[inline(always)]
	fn hash_as_receive_side_scaling_hash(self) -> u32
	{
		* self.reference().rss.as_ref()
	}
	
	/// Is the hash a flow director filter identifier?
	#[inline(always)]
	fn has_flow_director_filter_identifier(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR)
	}
	
	/// Flow director filter identifier.
	#[inline(always)]
	fn hash_as_flow_director_filter_identifier(self) -> u32
	{
		let flow_director = self.reference().fdir.as_ref();
		flow_director.hi
	}
	
	/// Is the hash a flow director hash and filter identifier?
	#[inline(always)]
	fn has_flow_director_hash_and_filter_identifier(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR_ID)
	}
	
	/// Flow director hash and filter identifier.
	#[inline(always)]
	fn hash_as_flow_director_hash_and_filter_identifier(self) -> (u16, u16)
	{
		let flow_director = self.reference().fdir.as_ref();
		let hash_and_identifier = flow_director._1._1.as_ref();
		(hash_and_identifier.hash, hash_and_identifier.id)
	}
	
	/// Is the hash a flow director flexible bytes?
	#[inline(always)]
	fn has_flow_director_flexible_bytes_high_and_low(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR_FLX)
	}
	
	/// Flow director flexible bytes.
	#[inline(always)]
	fn hash_as_flow_director_flexible_bytes_high_and_low(self) -> (u32, u32)
	{
		let flow_director = self.reference().fdir.as_ref();
		(flow_director.hi, flow_director._1.lo)
	}
	
	/// Hierarchical Scheduler flexible bytes.
	#[inline(always)]
	fn hash_as_hierarchical_scheduler_bytes_high_and_low(self) -> (u32, u32)
	{
		let scheduler = * self.reference().sched;
		(scheduler.hi, scheduler.lo)
	}
	
	/// User define tags.
	///
	/// See `rte_distributor_process()`.
	#[inline(always)]
	fn hash_as_user_defined_tags(self) -> u32
	{
		* self.reference().usr.as_ref()
	}
	
	/// Finds a segment at the offset given.
	///
	/// Returns the found segment and the reduced offset, ie as if offset from found segment.
	///
	/// Panics if `offset + buffer.len() > self.length()`.
	#[inline(always)]
	fn find_segment_at_offset(self, mut offset: u32) -> (NonNull<rte_mbuf>, u32)
	{
		let mut segment = self;
		while offset >= segment.data_length()
		{
			offset -= segment.data_length();
			segment = segment.next_segment().expect("Number of segments x each segment length != packet length; violation of expected packet state");
		}
		(segment, offset)
	}
	
	/// Writes (copies) `buffer.len()` data bytes into the packet from `buffer`, regardless of how many segments it has starting at the packet at `offset`.
	///
	/// Panics if `offset + buffer.len() > self.length()`.
	///
	/// `copy_from` must not overlap with the packet's data.
	#[inline(always)]
	fn write_even_if_non_contiguous(self, offset: u32, copy_from: &[u8])
	{
		let length = copy_from.len() as u32;
		
		debug_assert!(offset + length <= self.length(), "offset '{}' + copy_from.len() '{}' exceeds packet length '{}'", offset, length, self.length());
		
		if offset + length <= (self.data_length() as u32)
		{
			let destination = self.offset_into_data(self, offset);
			unsafe { rust_rte_memcpy(destination, copy_from.as_ptr() as *const c_void, length as usize) };
			return
		}
		
		let (mut segment, mut offset) = self.find_segment_at_offset(offset);

		if offset + length <= (segment.data_length() as u32)
		{
			let destination = segment.offset_into_data(offset);
			unsafe { rust_rte_memcpy(destination, copy_from.as_ptr() as *const c_void, length as usize) };
			return
		}
		
		let mut copy_from_offset: u32 = 0;
		let mut remaining_length = length;
		while remaining_length > 0
		{
			let length_of_this_copy =
			{
				let maximum_copy_length = (segment.data_length() as u32) - offset;
				
				if maximum_copy_length > remaining_length
				{
					remaining_length
				}
				else
				{
					maximum_copy_length
				}
			};
			
			{
				let destination = segment.offset_into_data(offset as usize);
				let copy_from_pointer = unsafe { copy_from.get_unchecked(copy_from_offset) };
				unsafe { rust_rte_memcpy(destination, copy_from_pointer as *const c_void, length_of_this_copy as usize) };
			}
			
			offset = 0;
			
			segment = segment.next_segment();
			remaining_length -= length_of_this_copy;
			copy_from_offset += length_of_this_copy;
		}

		Ok(())
	}
	
	/// Reads data bytes in a packet from `offset` of `buffer.len()` bytes.
	///
	/// If the packet is contiguous, or, the requested `offset` and `buffer.len()` is entirely withing it, returns a slice to the data inside the packet.
	///
	/// If the packet is not contiguous (it contains more than one segment), reads `buffer.len()` bytes and copies it into `buffer`; returns `buffer` as a slice.
	fn read_even_if_non_contiguous<'a>(self, length: u32, offset: u32, buffer: &'a mut [u8]) -> &'a mut [u8];
	
}

impl PrintInformation for NonNull<rte_mbuf>
{
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		const FirstNBytesOfPacketData: usize = 0;
		unsafe { rte_pktmbuf_dump(stderr as *mut FILE, self.as_ptr(), FirstNBytesOfPacketData) }
	}
}

impl PacketBuffer for NonNull<rte_mbuf>
{
}




fn read_even_if_non_contiguous<'a>(self, length: u32, offset: u32, buffer: &'a mut [u8]) -> &'a mut [u8]
{
	let length = buffer.len();
	let buffer_pointer = buffer.as_mut_ptr() as *mut c_void;
	let pointer_to_read_from: *const c_void = unsafe { rte_pktmbuf_read(self.as_ptr() as *const _, offset, length as u32, buffer_pointer) };
	debug_assert(pointer_to_read_from.is_not_null(), "pointer_to_read_from is null");
	
	if pointer_to_read_from == buffer_pointer
	{
		buffer
	}
	else
	{
		unsafe { from_raw_parts_mut(pointer_to_read_from as *mut u8, length) }
	}
	
	XXXXX
	
	/*
	static inline const void *rte_pktmbuf_read(const struct rte_mbuf *m,
uint32_t off, uint32_t len, void *buf)
{
if (likely!(off + len <= rte_pktmbuf_data_len(m)))
	return rte_pktmbuf_mtod_offset(m, char *, off);
else
	return __rte_pktmbuf_read(m, off, len, buf);
}
	
	*/
}

// Replicate read_even_if_non_contiguous

// Implement rte_memcpy in Rust.


//pub fn rust_rte_mbuf_data_iova(mb: *const rte_mbuf) -> rte_iova_t;
//pub fn rust_rte_mbuf_data_iova_default(mb: *const rte_mbuf) -> rte_iova_t;
//pub fn rust_rte_mbuf_from_indirect(mi: *mut rte_mbuf) -> *mut rte_mbuf;
//pub fn rust_rte_mbuf_prefetch_part1(m: *mut rte_mbuf);
//pub fn rust_rte_mbuf_prefetch_part2(m: *mut rte_mbuf);
//pub fn rust_rte_mbuf_raw_alloc(mp: *mut rte_mempool) -> *mut rte_mbuf;
//pub fn rust_rte_mbuf_raw_free(m: *mut rte_mbuf);
//pub fn rust_rte_mbuf_refcnt_read(m: *const rte_mbuf) -> u16;
//pub fn rust_rte_mbuf_refcnt_set(m: *mut rte_mbuf, new_value: u16);
//pub fn rust_rte_mbuf_refcnt_update(m: *mut rte_mbuf, value: i16) -> u16;
//pub fn rust_rte_mbuf_to_baddr(md: *mut rte_mbuf) -> *mut c_char;
//
//
///**
// * IPv4 fragmentation.
// *
// * This function implements the fragmentation of IPv4 packets.
// *
// * @param pkt_in
// *   The input packet.
// * @param pkts_out
// *   Array storing the output fragments.
// * @param nb_pkts_out
// *   Number of fragments.
// * @param mtu_size
// *   Size in bytes of the Maximum Transfer Unit (MTU) for the outgoing IPv4
// *   datagrams. This value includes the size of the IPv4 header.
// * @param pool_direct
// *   MBUF pool used for allocating direct buffers for the output fragments.
// * @param pool_indirect
// *   MBUF pool used for allocating indirect buffers for the output fragments.
// * @return
// *   Upon successful completion - number of output fragments placed
// *   in the pkts_out array.
// *   Otherwise - (-1) * errno.
// */
//int32_t rte_ipv4_fragment_packet(struct rte_mbuf *pkt_in,
//struct rte_mbuf **pkts_out,
//uint16_t nb_pkts_out, uint16_t mtu_size,
//struct rte_mempool *pool_direct,
//struct rte_mempool *pool_indirect);
