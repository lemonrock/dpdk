// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Memory layout iterator.
pub trait VirtualAddress
{
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
	fn io_virtual_address(self) -> Result<rte_iova_t, ()>;
	
	/// Using direct, physical or virtual addresses?
	#[inline(always)]
	fn io_virtual_address_mode() -> rte_iova_mode
	{
		unsafe { rte_eal_iova_mode() }
	}
	
	/// Locks a page to prevent swapping.
	#[inline(always)]
	fn lock_page_to_prevent_swapping(self) -> bool;
	
	/// If true, then the system is able to obtain physical addresses.
	///
	/// If false, then the system is using DMA addresses through an IOMMU.
	///
	/// Drivers based on uio will not load unless physical addresses are obtainable.
	/// It is only possible to get physical addresses when running as a privileged user.
	#[inline(always)]
	fn is_using_physical_addresses() -> bool
	{
		(unsafe { rte_eal_using_phys_addrs() }) != 0
	}
}

impl VirtualAddress for *const c_void
{
	#[inline(always)]
	fn io_virtual_address(self) -> Result<rte_iova_t, ()>
	{
		const RTE_BAD_IOVA: u64 = 0xFFFF_FFFF_FFFF_FFFF;
		
		let result = unsafe { rte_mem_virt2iova(self) };
		
		if result == RTE_BAD_IOVA
		{
			Err(())
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn lock_page_to_prevent_swapping(self) -> bool
	{
		match unsafe { rte_mem_lock_page(self) }
		{
			0 => true,
			
			negative if negative < 0 => false,
			
			illegal @ _ => panic!("Unexpected result '{}' from rte_mem_lock_page()", illegal),
		}
	}
}
