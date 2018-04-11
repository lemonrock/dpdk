// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_cryptodev_info
{
	pub driver_name: *const c_char,
	pub driver_id: u8,
	pub pci_dev: *mut rte_pci_device,
	pub feature_flags: u64,
	pub capabilities: *const rte_cryptodev_capabilities,
	pub max_nb_queue_pairs: c_uint,
	pub sym: rte_cryptodev_info__bindgen_ty_1,
}

impl Default for rte_cryptodev_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_cryptodev_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_cryptodev_info {{ driver_name: {:?}, pci_dev: {:?}, capabilities: {:?}, sym: {:?} }}", self.driver_name, self.pci_dev, self.capabilities, self.sym)
	}
}
