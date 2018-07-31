// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A packet buffer pool.
#[derive(Debug)]
pub struct PacketBufferPool(NonNull<rte_mempool>);

impl PacketBufferPool
{
	/// Create a new instance.
	///
	/// * `number_of_elements`: The number of elements in the mbuf pool. The optimum size (in terms of memory usage) for a mempool is when `n` is a power of two minus one: `n = (2^q - 1)`.
	/// * `application_private_size` must be aligned to `RTE_MBUF_PRIV_ALIGN`. The application private size of mbuf is a zone located between the rte_mbuf structure and the data buffer where an application can store data associated to a packet.
	/// * `data_room_size` is the amount of data that can be stored in a mbuf including the headroom (`RTE_PKTMBUF_HEADROOM`).
	///
	#[inline(always)]
	pub fn new(memory_zone_name: &str, number_of_elements: u32, per_core_object_cache_size: u32, application_private_size: u16, data_room_size: u16, numa_node_choice: NumaNodeChoice) -> Option<Self>
	{
		// rte_pktmbuf_pool_create takes a copy, so this doesn't need to hang around. Phew!
		let memory_zone_name = CString::new(memory_zone_name).expect("memory_zone_name contained an interior ASCII NUL");
		
		let result = unsafe { rte_pktmbuf_pool_create(memory_zone_name.as_ptr(), number_of_elements, per_core_object_cache_size, application_private_size, data_room_size, numa_node_choice.into()) };
		if unlikely!(result.is_null())
		{
			match LogicalCore::current_logical_core_error_number()
			{
				E::ENOSPC => None,
				E::ENOMEM => None,
				
				E_RTE::NO_CONFIG => panic!("No config"),
				E_RTE::SECONDARY => panic!("Secondary process"),
				E::EINVAL => panic!("cache size provided is too large, or priv_size is not aligned"),
				E::EEXIST => panic!("a memzone with the same name already exists"),
				
				illegal @ _ => panic!("Unexpected error code '{}' set by rte_pktmbuf_pool_create()", illegal),
			}
		}
		else
		{
			Some(PacketBufferPool(result))
		}
	}
	
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
	pub fn allocate(&self) -> Option<PacketBuffer>
	{
		let result = unsafe { rust_rte_pktmbuf_alloc(self.as_ptr()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			let packet_buffer = PacketBuffer(result);
			{
				let packet_buffer = packet_buffer.mutable_reference();
				packet_buffer.hash = 0;
				packet_buffer.timestamp = 0;
				packet_buffer.timesync = 0;
				packet_buffer.seqn = 0;
				packet_buffer.shinfo = null_mut();
			}
			
			Some(packet_buffer)
		}
	}
	
	/// Bulk allocate packet buffers.
	#[inline(always)]
	pub fn bulk_allocate(&self) -> Option<[PacketBuffer; BulkAllocateUsize]>
	{
		let mut mbufs: [*mut rte_mbuf; BulkAllocateUsize] = unsafe { uninitialized() };
		let result = unsafe { rust_rte_pktmbuf_alloc_bulk(self.as_ptr(), mbufs.as_mut_ptr(), BulkAllocate) };
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
	
	/// Put.
	#[inline(always)]
	pub(crate) fn put(self, packet_buffer: PacketBuffer)
	{
		unsafe { rust_rte_mempool_put(self.as_ptr(), packet_buffer.0.as_ptr() as *mut c_void) }
		//self.put_bulk(&[packet_buffer])
	}
	
//	/// Put in bulk.
//	#[inline(always)]
//	pub(crate) fn put_bulk(self, packet_buffers: &[PacketBuffer])
//	{
//		debug_assert!(packet_buffers.len() <= ::std::u32::MAX as usize, "packet_buffers.len() exceeds maximum ::std::u32::MAX '{}'", ::std::u32::MAX);
//
//		let cache = self.default_cache(LogicalCore::current());
//
//		unsafe fn __mempool_generic_put(mp: *mut rte_mempool, obj_table: *const *mut c_void, n: u32, cache: *mut rte_mempool_cache)
//		{
//			if unlikely!(cache.is_null() || n > RTE_MEMPOOL_CACHE_MAX_SIZE)
//			{
//				rust_rte_mempool_ops_enqueue_bulk(mp, obj_table, n);
//				return
//			}
//			let cache = { &mut * cache };
//
//			obj_table.copy_nonoverlapping(&mut cache.objs[cache.len as usize], n as usize);
//
//			cache.len += n;
//
//			if cache.len >= cache.flushthresh
//			{
//				rust_rte_mempool_ops_enqueue_bulk(mp, &cache.objs[cache.size as usize], cache.len - cache.size);
//				cache.len = cache.size;
//			}
//		}
//
//		unsafe { __mempool_generic_put(self.as_ptr(), packet_buffers.as_ptr() as *const c_void, packet_buffers.len() as u32, cache) }
//	}
//
//	#[inline(always)]
//	fn default_cache(self, logical_core: LogicalCore) -> *mut rte_mempool_cache
//	{
//		if self.reference().cache_size == 0 || logical_core.into() >= LogicalCore::Maximum
//		{
//			null_mut()
//		}
//		else
//		{
//			unsafe { self.reference().local_cache.offset(logical_core.into()) }
//		}
//	}
	
	#[inline(always)]
	fn mutable_reference<'a>(self) -> &'a mut rte_mempool
	{
		unsafe { & mut * self.as_ptr() }
	}
	
	#[inline(always)]
	fn reference<'a>(self) -> &'a rte_mempool
	{
		unsafe { & * self.as_ptr() }
	}
	
	#[inline(always)]
	fn as_ptr(self) -> *mut rte_mempool
	{
		self.0.as_ptr()
	}
}
