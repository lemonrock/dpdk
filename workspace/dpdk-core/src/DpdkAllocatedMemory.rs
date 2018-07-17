// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Memory allocated by DPDK.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkAllocatedMemory<T>(pub *mut T);

impl<T> Drop for DpdkAllocatedMemory<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rte_free(self.0 as *mut c_void) }
	}
}

impl<T> PrintAllInformation for DpdkAllocatedMemory<T>
{
	#[inline(always)]
	fn print_information_to_stream(stream: *mut FILE)
	{
		unsafe { rte_malloc_dump_stats(stream, null()) };
	}
}

impl<T> DpdkAllocatedMemory<T>
{
	/// Physical address of mapping.
	///
	/// Deprecated in DPDK.
	///
	/// Returns an error if not a valid physical address.
	#[deprecated(note = "please use `virtual_to_io_virtual_address()` instead")]
	#[inline(always)]
	pub fn physical_address(&self) -> Result<phys_addr_t, ()>
	{
		unsafe { transmute(self.virtual_to_io_virtual_address()) }
	}
	
	/// Returns the IO address of a virtual address obtained through `rte_malloc`.
	///
	/// Returns an error if not a valid address.
	///
	/// When the physical addressing mode (IOVA as a Physical Address) is in use, the translation from an IO Virtual Address (IOVA) to a physical address is a direct mapping, ie the same value. Otherwise, in virtual mode (IOVA as a Virtual Address), an IOMMU may do the translation.
	///
	/// Returns an error if not a valid address.
	#[inline(always)]
	pub fn virtual_to_io_virtual_address(&self) -> Result<rte_iova_t, ()>
	{
		let result = unsafe { rte_malloc_virt2iova(self.0 as *mut c_void) };
		if (result as i64) == -1
		{
			Err(())
		}
		else
		{
			Ok(result)
		}
	}
	
	/// Validates the size of this memory.
	///
	/// Returns None if not valid.
	#[inline(always)]
	pub fn validate(&self) -> Option<usize>
	{
		let mut size = unsafe { uninitialized() };
		let result = unsafe { rte_malloc_validate(self.0 as *mut c_void, &mut size) };
		if likely!(result == 0)
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
}
