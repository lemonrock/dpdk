// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_pci_driver
{
	pub next: rte_pci_driver_1,
	pub driver: rte_driver,
	pub bus: *mut rte_pci_bus,
	pub probe: pci_probe_t,
	pub remove: pci_remove_t,
	pub id_table: *const rte_pci_id,
	pub drv_flags: u32,
}

impl Default for rte_pci_driver
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_pci_driver
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_pci_driver {{ next: {:?}, driver: {:?}, bus: {:?}, probe: {:?}, remove: {:?}, id_table: {:?} }}", self.next, self.driver, self.bus, self.probe, self.remove, self.id_table)
	}
}
