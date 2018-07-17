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
		unsafe { rte_mempool_put(self.as_ptr(), packet_buffer.as_ptr()) }
	}
	
	/// Default cache.
	#[inline(always)]
	pub(crate) fn default_cache(self, logical_core: LogicalCore) -> *mut rte_mempool_cache
	{
		if self.reference().cache_size == 0 || logical_core.into() >= LogicalCore::Maximum
		{
			null_mut()
		}
		else
		{
			unsafe { self.reference().local_cache.offset(logical_core.into()) }
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn mutable_reference<'a>(self) -> &'a mut rte_mempool
	{
		unsafe { & mut * self.as_ptr() }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn reference<'a>(self) -> &'a rte_mempool
	{
		unsafe { & * self.as_ptr() }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn as_ptr(self) -> *mut rte_mempool
	{
		self.0.as_ptr()
	}
}
