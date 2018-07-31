// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a generic DPDK device; a sort of super-class.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkDevice<'a>(NonNull<rte_device>, PhantomData<&'a rte_device>);

impl<'a> DpdkDevice<'a>
{
	#[inline(always)]
	pub(crate) fn new(device: *mut rte_device) -> Self
	{
		debug_assert!(!device.is_null(), "device is null");
		
		DpdkDevice(unsafe { NonNull::new_unchecked(device) }, PhantomData)
	}
	
	/// Next.
	#[inline(always)]
	pub fn next(&self) -> Option<Self>
	{
		let next = self.reference().next.tqe_next;
		if next.is_null()
		{
			None
		}
		else
		{
			Some(DpdkDevice::new(next))
		}
	}
	
	/// NUMA node.
	#[inline(always)]
	pub fn numa_socket_id(&self) -> NumaNodeChoice
	{
		NumaNodeChoice::from_i32(self.reference().numa_node)
	}
	
	/// Name (does not exceed 64 bytes).
	#[inline(always)]
	pub fn name(&self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(self.reference().name) }
	}
	
	/// Generic DPDK driver.
	#[inline(always)]
	pub fn driver(&'a self) -> DpdkDriver<'a>
	{
		DpdkDriver(unsafe { NonNull::new_unchecked(self.reference().driver as *mut _) }, PhantomData)
	}
	
	#[inline(always)]
	fn reference(&self) -> &'a rte_device
	{
		unsafe { & * self.0.as_ptr() }
	}
}
