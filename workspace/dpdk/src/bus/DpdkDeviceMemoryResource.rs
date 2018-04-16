// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A DPDK device memory resource.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkDeviceMemoryResource<'a>(&'a rte_mem_resource, PhantomData<&'a [rte_mem_resource; 6]>);

impl<'a> DpdkDeviceMemoryResource<'a>
{
	/// Physical address.
	#[inline(always)]
	pub fn physical_address(&self) -> u64
	{
		self.0.phys_addr
	}
	
	/// Virtual address.
	#[inline(always)]
	pub fn virtual_address(&self) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked(self.0.addr as *mut u8) }
	}
	
	/// Length.
	#[inline(always)]
	pub fn length(&self) -> u64
	{
		self.0.len
	}
}
