// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An input-output port.
///
/// Unmapped on drop.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkPciInputOutputPort(UnsafeCell<rte_pci_ioport>);

impl Drop for DpdkPciInputOutputPort
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let result = unsafe { rte_eal_pci_ioport_unmap(self.handle()) };
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

impl DpdkPciInputOutputPort
{
	/// New
	pub fn new(dpdk_pci_device: &DpdkPciDevice, base_address_register: i32) -> Self
	{
		dpdk_pci_device.map_input_output_port(base_address_register)
	}
	
	/// Read.
	#[inline(always)]
	pub fn read(&self, read_into: &mut [u8], offset_into_port: isize)
	{
		unsafe { rte_eal_pci_ioport_read(self.handle(), read_into.as_mut_ptr() as *mut c_void, read_into.len(), offset_into_port as off_t) }
	}

	/// Write.
	#[inline(always)]
	pub fn write(&self, write_from: &[u8], offset_into_port: isize)
	{
		unsafe { rte_eal_pci_ioport_read(self.handle(), write_from.as_ptr() as *mut c_void, write_from.len(), offset_into_port as off_t) }
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_pci_ioport
	{
		self.0.get()
	}
}
