// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_vfio_clear_group(vfio_group_fd: c_int) -> c_int;
	pub fn rte_vfio_container_create() -> c_int;
	pub fn rte_vfio_container_destroy(container_fd: c_int) -> c_int;
	pub fn rte_vfio_container_dma_map(container_fd: c_int, vaddr: u64, iova: u64, len: u64) -> c_int;
	pub fn rte_vfio_container_dma_unmap(container_fd: c_int, vaddr: u64, iova: u64, len: u64) -> c_int;
	pub fn rte_vfio_container_group_bind(container_fd: c_int, iommu_group_num: c_int) -> c_int;
	pub fn rte_vfio_container_group_unbind(container_fd: c_int, iommu_group_num: c_int) -> c_int;
	pub fn rte_vfio_dma_map(vaddr: u64, iova: u64, len: u64) -> c_int;
	pub fn rte_vfio_dma_unmap(vaddr: u64, iova: u64, len: u64) -> c_int;
	pub fn rte_vfio_enable(modname: *const c_char) -> c_int;
	pub fn rte_vfio_get_container_fd() -> c_int;
	pub fn rte_vfio_get_group_fd(iommu_group_num: c_int) -> c_int;
	pub fn rte_vfio_get_group_num(sysfs_base: *const c_char, dev_addr: *const c_char, iommu_group_num: *mut c_int) -> c_int;
	pub fn rte_vfio_is_enabled(modname: *const c_char) -> c_int;
	pub fn rte_vfio_noiommu_is_enabled() -> c_int;
	pub fn rte_vfio_release_device(sysfs_base: *const c_char, dev_addr: *const c_char, fd: c_int) -> c_int;
	pub fn rte_vfio_setup_device(sysfs_base: *const c_char, dev_addr: *const c_char, vfio_dev_fd: *mut c_int, device_info: *mut vfio_device_info) -> c_int;
}
