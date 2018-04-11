// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_bus_dump(f: *mut FILE);
	pub fn rte_bus_find(start: *const rte_bus, cmp: rte_bus_cmp_t, data: *const c_void) -> *mut rte_bus;
	pub fn rte_bus_find_by_device(dev: *const rte_device) -> *mut rte_bus;
	pub fn rte_bus_find_by_name(busname: *const c_char) -> *mut rte_bus;
	pub fn rte_bus_get_iommu_class() -> rte_iova_mode;
	pub fn rte_bus_probe() -> c_int;
	pub fn rte_bus_register(bus: *mut rte_bus);
	pub fn rte_bus_scan() -> c_int;
	pub fn rte_bus_unregister(bus: *mut rte_bus);
}
