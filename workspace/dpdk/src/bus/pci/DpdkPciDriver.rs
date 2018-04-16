// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Wraps a DPDK driver.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkPciDriver(NonNull<rte_pci_driver>);

impl DpdkPciDriver
{
	/// Probe.
	#[inline(always)]
	pub fn probe() -> Vec<Self>
	{
		match unsafe { rte_eal_pci_probe() }
		{
			0 => (),
			negative if negative < 0 => panic!("Could not probe PCI bus, error code was '{}'", negative),

			illegal @ _ => panic!("Invalid result code '{}' from rte_eal_pci_probe()", illegal),
		};

		let pci_driver_list = unsafe { pci_driver_list };

		let first_element = pci_driver_list.tqh_first;
		let is_empty = first_element.is_null();
		let capacity = if is_empty
		{
			0
		}
		else
		{
			256
		};

		let mut drivers = Vec::with_capacity(capacity);

		let mut element = first_element;
		while !element.is_null()
		{
			drivers.push(DpdkPciDriver(unsafe { NonNull::new_unchecked(element) }));
			let element_value = unsafe { (*element) };
			element = element_value.next.tqe_next;
		}
		drivers.shrink_to_fit();

		drivers
	}
	
	/// Name.
	#[inline(always)]
	pub fn name(&self) -> &'static str
	{
		unsafe
		{
			let c_name = CStr::from_ptr(self.deref().driver.name);
			c_name.to_str().expect("Bad name from DPDK")
		}
	}
	
	/// Flags.
	#[inline(always)]
	pub fn flags(&self) -> DpdkPciDriverFlags
	{
		DpdkPciDriverFlags::from_bits_truncate(self.deref().drv_flags)
	}
	
	/// Register.
	#[inline(always)]
	pub fn register(&self)
	{
		unsafe { rte_eal_pci_register(self.handle()) }
	}
	
	/// Unregister.
	#[inline(always)]
	pub fn unregister(&self)
	{
		unsafe { rte_eal_pci_unregister(self.handle()) }
	}
	
	#[inline(always)]
	fn deref(&self) -> &rte_pci_driver
	{
		unsafe { & * self.handle() }
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_pci_driver
	{
		self.0.as_ptr()
	}
}
