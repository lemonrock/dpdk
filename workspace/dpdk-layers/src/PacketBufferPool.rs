// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct PacketBufferPool(NonNull<rte_mempool>);

impl PacketBufferPool
{
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
