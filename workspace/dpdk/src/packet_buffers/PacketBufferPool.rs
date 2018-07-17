// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



pub type PacketBufferPool = NonNull<rte_mempool>;

pub trait PacketBufferPoolExt
{
	/// Put.
	#[inline(always)]
	fn put(self, packet_buffer: PacketBuffer)
	{
		unsafe { rte_mpool_put(self.mutable_pointer(), packet_buffer.as_ptr()) }
	}
	
	/// Put in bulk.
	#[inline(always)]
	fn put_bulk(self, packet_buffers: &[PacketBuffer])
	{
		debug_assert!(packet_buffers.len() <= ::std::u32::MAX as usize, "packet_buffers.len() exceeds maximum ::std::u32::MAX '{}'", ::std::u32::MAX);
		
		let cache = self.default_cache(LogicalCore::current());
		
		unsafe { rust___mempool_generic_put(self.mutable_pointer(), packet_buffers.as_ptr() as *const c_void, packet_buffers.len() as u32, cache) }
	}
	
	/// Default cache.
	#[inline(always)]
	fn default_cache(self, logical_core: LogicalCore) -> *mut rte_mempool_cache
	{
		if self.reference().cache_size == 0 || logical_core.as_u32() >= LogicalCore::Maximum
		{
			null_mut()
		}
		else
		{
			unsafe { self.reference().local_cache.offset(logical_core.as_u32()) }
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn reference<'a>(self) -> &'a rte_mempool;
	
	#[doc(hidden)]
	#[inline(always)]
	fn mutable_pointer(self) -> *mut rte_mempool;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketBufferPool(pub *mut rte_mempool);

pub const BulkAllocate: u32 = 32;
const BulkAllocateUsize: usize = BulkAllocate as usize;

impl PacketBufferPool
{
	#[inline(always)]
	pub fn new(memoryZoneName: &str, numberOfElements: u32, perCoreObjectCacheSize: u32, applicationPrivateSize: u16, dataRoomSize: u16, numa_node_choice: NumaNodeChoice) -> Option<PacketBufferPool>
	{
		// rte_pktmbuf_pool_create takes a copy, so this doesn't need to hang around. Phew!
		let memoryZoneName = CString::new(memoryZoneName).expect("memoryZoneName contained an interior ASCII NUL");

		let result = unsafe { rte_pktmbuf_pool_create(memoryZoneName.as_ptr(), numberOfElements, perCoreObjectCacheSize, applicationPrivateSize, dataRoomSize, numa_node_choice.into()) };
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

	#[inline(always)]
	pub fn memoryPool(&self) -> *mut rte_mempool
	{
		self.0
	}

	#[inline]
	pub fn dataRoomSize(&self) -> u16
	{
		unsafe { rust_rte_pktmbuf_data_room_size(self.memoryPool()) }
	}

	#[inline]
	pub fn allocate(&self) -> Option<PacketBuffer>
	{
		let result = unsafe { rust_rte_pktmbuf_alloc(self.memoryPool()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some(PacketBuffer(result))
		}
	}

	#[inline(always)]
	pub fn bulkAllocate(&self) -> Option<[PacketBuffer; BulkAllocateUsize]>
	{
		let mut mbufs: [*mut rte_mbuf; BulkAllocateUsize] = unsafe { uninitialized() };
		let result = unsafe { rust_rte_pktmbuf_alloc_bulk(self.memoryPool(), mbufs.as_mut_ptr(), BulkAllocate) };
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
