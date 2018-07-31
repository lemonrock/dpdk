// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A wrapper around `mmap()`.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkPciMappedResource
{
	requested_address: NonNull<u8>,
	size: usize,
	mapped_area: NonNull<u8>,
}

impl Drop for DpdkPciMappedResource
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { pci_unmap_resource(self.requested_address.as_ptr() as *mut c_void, self.size) }
	}
}

impl DpdkPciMappedResource
{
	/// Creates a new mapping.
	#[inline(always)]
	pub fn new(requested_address: NonNull<u8>, size: usize, file_descriptor: RawFd, offset: off_t, additional_flags: i32) -> Result<Self, ()>
	{
		let result = unsafe { pci_map_resource(requested_address.as_ptr() as *mut c_void, file_descriptor, offset, size, additional_flags) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok
			(
				Self
				{
					requested_address,
					size,
					mapped_area: unsafe { NonNull::new_unchecked(result as *mut u8) },
				}
			)
		}
	}
	
	/// Pointer to mapped area.
	#[inline(always)]
	pub fn mapped_area(&self) -> NonNull<u8>
	{
		self.mapped_area
	}
}
