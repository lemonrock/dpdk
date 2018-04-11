// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_bus
{
	pub next: rte_bus__bindgen_ty_1,
	pub name: *const c_char,
	pub scan: rte_bus_scan_t,
	pub probe: rte_bus_probe_t,
	pub find_device: rte_bus_find_device_t,
	pub plug: rte_bus_plug_t,
	pub unplug: rte_bus_unplug_t,
	pub parse: rte_bus_parse_t,
	pub conf: rte_bus_conf,
	pub get_iommu_class: rte_bus_get_iommu_class_t,
}

impl Default for rte_bus
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bus
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_bus {{ next: {:?}, name: {:?}, conf: {:?} }}", self.next, self.name, self.conf)
	}
}
