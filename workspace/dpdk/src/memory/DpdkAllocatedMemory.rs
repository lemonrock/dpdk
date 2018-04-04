// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkAllocatedMemory<T>(pub *mut T);

impl<T> Drop for DpdkAllocatedMemory<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ::dpdk_sys::rte_free(self.0 as *mut c_void) }
	}
}

impl<T> DpdkAllocatedMemory<T>
{	
	#[inline(always)]
	pub fn physicalAddress(&self) -> phys_addr_t
	{
		unsafe { ::dpdk_sys::rte_malloc_virt2phy(self.0 as *mut c_void) }
	}
	
	#[inline(always)]
	pub fn validate(&self) -> Option<usize>
	{
		let mut size = unsafe { uninitialized() };
		let result = unsafe { ::dpdk_sys::rte_malloc_validate(self.0 as *mut c_void, &mut size) };
		if likely(result == 0)
		{
			Some(size)
		}
		else
		{
			forget(size);
			
			match result
			{
				-1 => None,
			
				illegal @ _ => panic!("Unexpected result '{}' from rte_malloc_validate()", illegal),
			}
		}
	}
	
	// Page must be locked
	#[inline(always)]
	pub fn physicalAddressForAnyMemory(virtualAddress: *const c_void) -> phys_addr_t
	{
		unsafe { ::dpdk_sys::rte_mem_virt2phy(virtualAddress) }
	}
	
	#[inline(always)]
	pub fn lockPageToPreventSwapping(virtualAddress: *const c_void) -> bool
	{
		match unsafe { ::dpdk_sys::rte_mem_lock_page(virtualAddress) }
		{
			0 => true,
			
			negative if negative < 0 => false,
			
			illegal @ _ => panic!("Unexpected result '{}' from rte_mem_lock_page()", illegal),
		}
	}
}
