// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An iterator over DPDK device memory resources.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkDeviceMemoryResources<'a>(&'a [rte_mem_resource; 6], PhantomData<&'a rte_pci_device>, usize);

impl<'a> Iterator for DpdkDeviceMemoryResources<'a>
{
	type Item = DpdkDeviceMemoryResource<'a>;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.2 == 6
		{
			return None
		}
		
		let current = unsafe { self.0.get_unchecked(self.2) };
		if (*current).addr.is_null()
		{
			return None
		}
		
		self.2 += 1;
		
		Some(DpdkDeviceMemoryResource(current, PhantomData))
	}
}
