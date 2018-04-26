// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Memory layout iterator.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryLayout
{
	table: NonNull<rte_memseg>,
	index: usize,
}

impl Iterator for MemoryLayout
{
	type Item = MemoryLayoutSegment;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.index == RTE_MAX_MEMSEG
		{
			return None;
		}
		
		let current = unsafe { NonNull::new_unchecked(self.table.as_ptr().offset(self.index as isize)) };
		if unsafe { current.as_ref() }._2.addr.is_null()
		{
			return None;
		}
		
		self.index += 1;
		
		MemorySegment(current)
	}
}

impl PrintAllInformation for MemoryLayout
{
	#[inline(always)]
	fn print_information_to_stream(stream: *mut FILE)
	{
		unsafe { rte_dump_physmem_layout(stream) };
	}
}

impl MemoryLayout
{
	/// Creates a new memory layout segment iterator.
	///
	/// Very cheap.
	#[inline(always)]
	pub fn new_layout_segment_iterator() -> Self
	{
		let result = unsafe { rte_eal_get_physmem_layout() };
		if unlikely(result.is_null())
		{
			panic!("Could not obtain memory layout; something is seriously wrong inside DPDK");
		}
		
		Self
		{
			table: unsafe { NonNull::new_unchecked(result) },
			index: 0,
		}
	}
	
	/// Configured physical memory size in bytes.
	#[inline(always)]
	pub fn configured_physical_memory_size_in_bytes() -> usize
	{
		unsafe { rte_eal_get_physmem_size() }
	}
	
	/// Configured number of memory channels.
	///
	/// Returns None if differs across memory segments or devices.
	#[inline(always)]
	pub fn configured_number_of_memory_channels() -> Option<MemoryChannels>
	{
		let channels = unsafe { rte_memory_get_nchannel() };
		if channels == 0
		{
			return None
		}
		if channels > 4
		{
			panic!("Invalid number of memory channels '{}'", channels)
		}
		Some(unsafe { transmute(channels) })
	}
	
	/// Configured number of memory ranks.
	///
	/// Returns None if differs across memory segments or devices.
	#[inline(always)]
	pub fn configured_number_of_memory_ranks() -> Option<MemoryRanks>
	{
		let ranks = unsafe { rte_memory_get_nrank() };
		if ranks == 0
		{
			return None
		}
		if ranks > 16
		{
			panic!("Invalid number of memory ranks '{}'", ranks)
		}
		Some(unsafe { transmute(ranks) })
	}
	
	/// Does the process have access to huge pages?
	#[inline(always)]
	pub fn has_huge_pages() -> bool
	{
		unsafe { rte_eal_has_hugepages() != 0 }
	}
	
	/// Returns the IO address of a virtual address; need not have been allocated using `rte_malloc`.
	///
	/// Returns an error if not a valid address.
	///
	/// When the physical addressing mode (IOVA as a Physical Address) is in use, the translation from an IO Virtual Address (IOVA) to a physical address is a direct mapping, ie the same value. Otherwise, in virtual mode (IOVA as a Virtual Address), an IOMMU may do the translation.
	///
	/// Returns an error if not a valid address.
	///
	/// Page should be locked before relying on the result.
	#[inline(always)]
	pub fn io_virtual_address(virtual_address: *const c_void) -> Result<rte_iova_t, ()>
	{
		let result = unsafe { rte_mem_virt2iova(virtual_address) };
		if (result as i64) == -1
		{
			Err(())
		}
		else
		{
			Ok(result)
		}
	}
	
	/// Locks a page to prevent swapping.
	#[inline(always)]
	pub fn lock_page_to_prevent_swapping(virtual_address: *const c_void) -> bool
	{
		match unsafe { rte_mem_lock_page(virtual_address) }
		{
			0 => true,
			
			negative if negative < 0 => false,
			
			illegal @ _ => panic!("Unexpected result '{}' from rte_mem_lock_page()", illegal),
		}
	}
	
	/// If true, then the system is able to obtain physical addresses.
	///
	/// If false, then the system is using DMA addresses through an IOMMU.
	///
	/// Drivers based on uio will not load unless physical addresses are obtainable.
	/// It is only possible to get physical addresses when running as a privileged user.
	#[inline(always)]
	pub fn is_using_physical_addresses() -> bool
	{
		(unsafe { rte_eal_using_phys_addrs() }) != 0
	}
	
	/// Using direct, physical or virtual addresses?
	#[inline(always)]
	pub fn io_virtual_address_mode() -> rte_iova_mode
	{
		unsafe { rte_eal_iova_mode() }
	}
}
