// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_eal_cleanup() -> c_int;
	pub fn rte_eal_compare_pci_addr(addr: *const rte_pci_addr, addr2: *const rte_pci_addr) -> c_int;
	pub fn rte_eal_create_uio_dev() -> c_int;
	pub fn rte_eal_dev_attach(name: *const c_char, devargs: *const c_char) -> c_int;
	pub fn rte_eal_dev_detach(dev: *mut rte_device) -> c_int;
	pub fn rte_eal_get_configuration() -> *mut rte_config;
	pub fn rte_eal_get_lcore_state(slave_id: c_uint) -> rte_lcore_state_t;
	pub fn rte_eal_get_physmem_layout() -> *const rte_memseg;
	pub fn rte_eal_get_physmem_size() -> u64;
	pub fn rte_eal_has_hugepages() -> c_int;
	pub fn rte_eal_has_pci() -> c_int;
	pub fn rte_eal_hotplug_add(busname: *const c_char, devname: *const c_char, devargs: *const c_char) -> c_int;
	pub fn rte_eal_hotplug_remove(busname: *const c_char, devname: *const c_char) -> c_int;
	pub fn rte_eal_hpet_init(make_default: c_int) -> c_int;
	pub fn rte_eal_init(argc: c_int, argv: *mut *mut c_char) -> c_int;
	pub fn rte_eal_iopl_init() -> c_int;
	pub fn rte_eal_iova_mode() -> rte_iova_mode;
	pub fn rte_eal_lcore_role(lcore_id: c_uint) -> rte_lcore_role_t;
	pub fn rte_eal_mbuf_default_mempool_ops() -> *const c_char;
	pub fn rte_eal_mbuf_user_pool_ops() -> *const c_char;
	pub fn rte_eal_primary_proc_alive(config_file_path: *const c_char) -> c_int;
	pub fn rte_eal_process_type() -> rte_proc_type_t;
	pub fn rte_eal_remote_launch(f: lcore_function_t, arg: *mut c_void, slave_id: c_uint) -> c_int;
	pub fn rte_eal_using_phys_addrs() -> c_int;
	pub fn rte_eal_vfio_intr_mode() -> rte_intr_mode;
	pub fn rte_eal_wait_lcore(slave_id: c_uint) -> c_int;
}
