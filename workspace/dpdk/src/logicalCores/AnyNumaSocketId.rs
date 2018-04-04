// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait AnyNumaSocketId
{
	#[inline(always)]
	fn isAny(&self) -> bool;
	
	#[inline(always)]
	fn as_c_int(&self) -> c_int;
	
	#[inline(always)]
	fn as_c_uint(&self) -> c_uint;
	
	#[inline(always)]
	fn as_int32_t(&self) -> int32_t;
	
	const CacheLineSize: u32 = 64;
	
	#[inline(always)]
	fn allocate<T>(&self, typeOfMemory: Option<ConstCStr>, size: usize, alignment: Option<PowerOfTwoThirtyTwoBit>) -> Option<DpdkAllocatedMemory<T>>
	{
		let alignment = alignment.as_u32();
		debug_assert!(alignment == 0 || alignment >= Self::CacheLineSize, "alignment must be greater than or equal to cache line size '{}', not '{}'", Self::CacheLineSize, alignment);
		
		let result = unsafe { ::dpdk_sys::rte_malloc_socket(typeOfMemoryX(typeOfMemory), size, alignment, self.as_c_int()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}

	#[inline(always)]
	fn zeroAllocate<T>(&self, typeOfMemory: Option<ConstCStr>, size: usize, alignment: Option<PowerOfTwoThirtyTwoBit>) -> Option<DpdkAllocatedMemory<T>>
	{
		let alignment = alignment.as_u32();
		debug_assert!(alignment == 0 || alignment >= Self::CacheLineSize, "alignment must be greater than or equal to cache line size '{}', not '{}'", Self::CacheLineSize, alignment);
		
		let result = unsafe { ::dpdk_sys::rte_zmalloc_socket(typeOfMemoryX(typeOfMemory), size, alignment, self.as_c_int()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}

	#[inline(always)]
	fn cAllocate<T>(&self, typeOfMemory: Option<ConstCStr>, numberOfElements: usize, sizeOfAnElement: usize, alignment: Option<PowerOfTwoThirtyTwoBit>) -> Option<DpdkAllocatedMemory<T>>
	{
		let alignment = alignment.as_u32();
		debug_assert!(alignment == 0 || alignment >= Self::CacheLineSize, "alignment must be greater than or equal to cache line size '{}', not '{}'", Self::CacheLineSize, alignment);
		
		let result = unsafe { ::dpdk_sys::rte_calloc_socket(typeOfMemoryX(typeOfMemory), numberOfElements, sizeOfAnElement, alignment, self.as_c_int()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}
}

impl AnyNumaSocketId for Option<NumaSocketId>
{	
	#[inline(always)]
	fn isAny(&self) -> bool
	{
		true
	}
	
	#[inline(always)]
	fn as_c_int(&self) -> c_int
	{
		SOCKET_ID_ANY as c_int
	}
	
	// Weird
	#[inline(always)]
	fn as_c_uint(&self) -> c_uint
	{
		0xFFFF_FFFF
	}
	
	#[inline(always)]
	fn as_int32_t(&self) -> int32_t
	{
		SOCKET_ID_ANY as int32_t
	}
}
