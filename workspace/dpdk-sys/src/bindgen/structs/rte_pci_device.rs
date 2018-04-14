// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_pci_device
{
	pub next: rte_pci_device_1,
	pub device: rte_device,
	pub addr: rte_pci_addr,
	pub id: rte_pci_id,
	pub mem_resource: [rte_mem_resource; 6usize],
	pub intr_handle: rte_intr_handle,
	pub driver: *mut rte_pci_driver,
	pub max_vfs: u16,
	pub kdrv: rte_kernel_driver,
	pub name: [c_char; 18usize],
}

impl Default for rte_pci_device
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_pci_device
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_pci_device {{ next: {:?}, device: {:?}, addr: {:?}, id: {:?}, mem_resource: {:?}, intr_handle: {:?}, driver: {:?}, kdrv: {:?}, name: {:?} }}", self.next, self.device, self.addr, self.id, self.mem_resource, self.intr_handle, self.driver, self.kdrv, self.name)
	}
}
