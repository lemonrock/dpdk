// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Driver(pub *mut rte_pci_driver);

impl Driver
{
	#[inline(always)]
	pub fn probe() -> Vec<Driver>
	{
		match unsafe { ::dpdk_sys::rte_eal_pci_probe() }
		{
			0 => (),
			negative if negative < 0 => panic!("Could not probe PCI bus, error code was '{}'", negative),
			
			illegal @ _ => panic!("Invalid result code '{}' from rte_eal_pci_probe()", illegal),
		};
		
		let pciDriverList = unsafe { ::dpdk_sys::pci_driver_list };
				
		let firstElement = pciDriverList.tqh_first;
		let isEmpty = firstElement.is_null();
		let capacity = if isEmpty
		{
			0
		}
		else
		{
			256
		};
	
		let mut drivers = Vec::with_capacity(capacity);
	
		let mut element = firstElement;
		while !element.is_null()
		{
			drivers.push(Driver(element));
			let elementValue = unsafe { (*element) };
			element = elementValue.next.tqe_next;
		}
		drivers.shrink_to_fit();
		
		drivers
	}
	
	#[inline(always)]
	fn deref(&self) -> rte_pci_driver
	{
		unsafe { (*self.0) }
	}
	
	#[inline(always)]
	pub fn name(&self) -> &'static str
	{
		unsafe
		{
			let cName = CStr::from_ptr(self.deref().driver.name);
			cName.to_str().expect("Bad name from DPDK")
		}
	}
	
	#[inline(always)]
	pub fn flags(&self) -> DriverFlags
	{
		DriverFlags::from_bits_truncate(self.deref().drv_flags)
	}
	
	#[inline(always)]
	pub fn register(&self)
	{
		unsafe { ::dpdk_sys::rte_eal_pci_register(self.0) }
	}
	
	#[inline(always)]
	pub fn unregister(&self)
	{
		unsafe { ::dpdk_sys::rte_eal_pci_unregister(self.0) }
	}
}
