// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct rte_vdev_driver
{
	pub next: rte_vdev_driver__bindgen_ty_1,
	pub driver: rte_driver,
	pub probe: rte_vdev_probe_t,
	pub remove: rte_vdev_remove_t,
}

impl Default for rte_vdev_driver
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
