// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_pci_bus
{
	pub bus: rte_bus,
	pub device_list: rte_pci_device_list,
	pub driver_list: rte_pci_driver_list,
}

impl Default for rte_pci_bus
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_pci_bus
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_pci_bus {{ bus: {:?}, device_list: {:?}, driver_list: {:?} }}", self.bus, self.device_list, self.driver_list)
	}
}
