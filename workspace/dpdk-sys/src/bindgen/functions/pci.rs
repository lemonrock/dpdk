// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn pci_map_resource(requested_addr: *mut c_void, fd: c_int, offset: off_t, size: usize, additional_flags: c_int) -> *mut c_void;
	pub fn pci_unmap_resource(requested_addr: *mut c_void, size: usize);
	pub fn rte_pci_addr_cmp(addr: *const rte_pci_addr, addr2: *const rte_pci_addr) -> c_int;
	pub fn rte_pci_addr_parse(str: *const c_char, addr: *mut rte_pci_addr) -> c_int;
	pub fn rte_pci_device_name(addr: *const rte_pci_addr, output: *mut c_char, size: usize);
	pub fn rte_pci_dump(f: *mut FILE);
	pub fn rte_pci_get_sysfs_path() -> *const c_char;
	pub fn rte_pci_ioport_map(dev: *mut rte_pci_device, bar: c_int, p: *mut rte_pci_ioport) -> c_int;
	pub fn rte_pci_ioport_read(p: *mut rte_pci_ioport, data: *mut c_void, len: usize, offset: off_t);
	pub fn rte_pci_ioport_unmap(p: *mut rte_pci_ioport) -> c_int;
	pub fn rte_pci_ioport_write(p: *mut rte_pci_ioport, data: *const c_void, len: usize, offset: off_t);
	pub fn rte_pci_map_device(dev: *mut rte_pci_device) -> c_int;
	pub fn rte_pci_read_config(device: *const rte_pci_device, buf: *mut c_void, len: usize, offset: off_t) -> c_int;
	pub fn rte_pci_register(driver: *mut rte_pci_driver);
	pub fn rte_pci_unmap_device(dev: *mut rte_pci_device);
	pub fn rte_pci_unregister(driver: *mut rte_pci_driver);
	pub fn rte_pci_write_config(device: *const rte_pci_device, buf: *const c_void, len: usize, offset: off_t) -> c_int;
}
