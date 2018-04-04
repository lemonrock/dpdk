// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputOutputPort(rte_pci_ioport);

impl Drop for InputOutputPort
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let result = unsafe { ::dpdk_sys::rte_eal_pci_ioport_unmap(&mut self.0) };
		if likely(result == 0)
		{
			return;
		}
		match result
		{
			negative if negative < 0 => panic!("Could not unmap PCI I/O Port"),
			
			_ => panic!("rte_eal_pci_ioport_unmap() returned illegal result '{}'", result),
		}
	}
}

impl InputOutputPort
{
	#[inline(always)]
	pub fn new(data: rte_pci_ioport) -> Self
	{
		InputOutputPort(data)
	}
	
	#[inline(always)]
	pub fn read(&mut self, readInto: &mut [u8], offsetIntoPort: isize)
	{
		unsafe { ::dpdk_sys::rte_eal_pci_ioport_read(&mut self.0, readInto.as_mut_ptr() as *mut c_void, readInto.len(), offsetIntoPort as off_t) }
	}

	#[inline(always)]
	pub fn write(&mut self, writeFrom: &[u8], offsetIntoPort: isize)
	{
		unsafe { ::dpdk_sys::rte_eal_pci_ioport_read(&mut self.0, writeFrom.as_ptr() as *mut c_void, writeFrom.len(), offsetIntoPort as off_t) }
	}
}
