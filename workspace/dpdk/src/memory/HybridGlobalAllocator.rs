// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a Rust (memory) allocator, ie a pluggable `malloc`, to replace the standard Rust memory allocator.
///
/// It is hybrid because it is designed to be used both before and after DPDK is initialized. Memory assigned before DPDK is initialized is ***NEVER*** freed and is ***limited to 1Mb***. This includes memory `realloc'd`, but does not include memory of size zero (0). For DPDK, alignments are rescaled to be a minimum of cache line size. If and when const generics land (or alloca) it should be possible to enable configuration of a different memory limit than 1Mb.
///
/// Alignments should not exceed 2^31 (this is quite unlikely).
///
/// To use it, add the following code to your `src/main.rs`:-
///
/// ```
/// #![feature(const_fn)]
/// #![feature(global_allocator)]
///
/// #[global_allocator] static ALLOCATOR: HybridGlobalAllocator = HybridGlobalAllocator::new();
/// ```
pub struct HybridGlobalAllocator
{
	next: AtomicUsize,
	dpdk_configured: bool,
	fixed_memory: [u8; HybridGlobalAllocator::MemoryLimitInBytes],
}

macro_rules! allocate
{
	($self: expr, $layout: ident, $dpdk_allocate_function: path) =>
	{
		{
			let size = $layout.size();
			
			if unlikely(size == 0)
			{
				return $self.allocate_zero_sized()
			}
			
			let alignment = $layout.align();
			
			if likely(self.dpdk_configured)
			{
				let dpdk_alignment = dpdk_alignment!(alignment);
				let result = unsafe { $dpdk_allocate_function(null(), size, dpdk_alignment, Self::current_numa_node()) };
				if unlikely(result.is_null())
				{
					Self::out_of_memory()
				}
				else
				{
					result as _
				}
			}
		
			$self.fixed_memory_allocate(size, alignment)
		}
	}
}

macro_rules! dpdk_alignment
{
	($alignment: ident) =>
	{
		{
			const DpdkCacheLineSize: usize = 64;
			
			if alignment < DpdkCacheLineSize
			{
				DpdkCacheLineSize as u32
			}
			else if unlikely(alignment > ::std::u32::MAX as usize)
			{
				return Self::out_of_memory()
			}
			else
			{
				alignment as u32
			}
		}
	}
}

unsafe impl GlobalAlloc for HybridGlobalAllocator
{
	#[inline(always)]
	unsafe fn alloc(&self, layout: Layout) -> *mut Opaque
	{
		allocate!(self, layout, rte_malloc_socket)
	}
	
	#[inline(always)]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut Opaque
	{
		allocate!(self, layout, rte_zmalloc_socket)
	}
	
	#[inline(always)]
	unsafe fn realloc(&self, ptr: *mut Opaque, layout: Layout, new_size: usize) -> *mut Opaque
	{
		let alignment = layout.align();
		
		if unlikely(self.is_zero_size_or_fixed_memory_pointer(ptr, layout))
		{
			let new_layout = Layout::from_size_align_unchecked(new_size, alignment);
			let new_pointer = self.alloc(new_layout);
			if new_pointer.is_null()
			{
				return Self::out_of_memory()
			}
			else
			{
				copy_nonoverlapping(ptr as *const u8, new_pointer as *mut u8, min(layout.size(), new_size));
				self.fixed_memory_free(ptr, layout);
				new_pointer
			}
		}
		else
		{
			if unlikely(new_size == 0)
			{
				self.dpdk_free(ptr);
				self.allocate_zero_sized()
			}
			else
			{
				let dpdk_alignment = dpdk_alignment!(alignment);
				unsafe { rte_realloc(ptr as *mut _, new_size, dpdk_alignment) }
			}
		}
	}
	
	#[inline(always)]
	unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout)
	{
		if unlikely(self.is_zero_size_or_fixed_memory_pointer(ptr, layout))
		{
			self.fixed_memory_free(ptr, layout)
		}
		else
		{
			self.dpdk_free(ptr)
		}
	}
}

impl HybridGlobalAllocator
{
	const MemoryLimitInBytes: usize = 1024 * 1024;
	
	#[inline(always)]
	pub const fn new() -> Self
	{
		Self
		{
			next: AtomicUsize::new(0),
			dpdk_configured: false,
			fixed_memory: [0; Self::MemoryLimitInBytes],
		}
	}
	
	#[inline(always)]
	fn fixed_memory_allocate(&self, size: usize, alignment: usize) -> *mut Opaque
	{
		let mut next_as_was = self.next.load(Relaxed);
		let mut next_aligned;
		loop
		{
			// TODO: Optimize, as alignment is always a power of two.
			next_aligned = ((next_as_was + alignment - 1) / alignment) * alignment;
			let next = next_aligned + size;
			if unlikely(next >= HybridGlobalAllocator::MemoryLimitInBytes)
			{
				return Self::out_of_memory()
			}
			match self.next.compare_exchange(next_as_was, next, AcqRel, Acquire)
			{
				Ok(_) => break,
				Err(was) =>
				{
					next_as_was = was;
				}
			}
		}
		
		((self.fixed_memory.as_ptr() as usize) + next_aligned) as *mut Opaque
	}
	
	#[inline(always)]
	fn fixed_memory_free(&self, _pointer: *mut Opaque, _layout: Layout)
	{
	}
	
	#[inline(always)]
	fn dpdk_free(&self, pointer: *mut Opaque)
	{
		unsafe { rte_free(pointer as *mut _) }
	}
	
	#[inline(always)]
	fn is_zero_size_or_fixed_memory_pointer(&self, ptr: *mut Opaque, layout: Layout) -> bool
	{
		layout.size() == 0 || self.is_fixed_memory_pointer(ptr)
	}
	
	#[inline(always)]
	fn is_fixed_memory_pointer(&self, ptr: *mut Opaque) -> bool
	{
		let pointer = ptr as usize;
		let fixed_memory_inclusive_start = self.fixed_memory.as_ptr() as usize;
		let fixed_memory_exclusive_end = fixed_memory_inclusive_start + Self::MemoryLimitInBytes;
		pointer >= fixed_memory_inclusive_start && pointer < fixed_memory_exclusive_end
	}
	
	#[inline(always)]
	fn current_numa_node() -> i32
	{
		NumaNode::numa_node_and_hyper_thread().0 as i32
	}
	
	#[inline(always)]
	const fn out_of_memory() -> *mut Opaque
	{
		0 as _
	}
	
	#[inline(always)]
	fn allocate_zero_sized(&self)
	{
		self.fixed_memory.as_ptr() as *mut Opaque
	}
}
