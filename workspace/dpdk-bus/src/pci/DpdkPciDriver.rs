// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Wraps a DPDK driver.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkPciDriver(NonNull<rte_pci_driver>);

impl DpdkPciDriver
{
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
		unsafe { rte_pci_register(self.handle()) }
	}
	
	/// Unregister.
	#[inline(always)]
	pub fn unregister(&self)
	{
		unsafe { rte_pci_unregister(self.handle()) }
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
