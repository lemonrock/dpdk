// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents `/sys`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct SysPath(PathBuf);

impl Default for SysPath
{
	#[inline(always)]
	fn default() -> Self
	{
		SysPath(PathBuf::from("/sys"))
	}
}

impl SysPath
{
	/// Is this a NUMA-based machine?
	#[inline(always)]
	pub fn is_a_numa_machine(&self) -> bool
	{
		self.numa_nodes_parent_path().exists()
	}
	
	/// Is this a NUMA node (assuming we're on a NUMA-based machine)?
	///
	/// Note that this might be a fake NUMA node, ie one lacking any hyper threads.
	#[inline(always)]
	pub fn is_a_numa_node(&self, numa_node: u8) -> bool
	{
		self.numa_node_folder_path(numa_node).exists()
	}
	
	/// A hyper thread file.
	#[inline(always)]
	pub fn hyper_thread_path(&self, hyper_thread: HyperThread, file_name: &str) -> PathBuf
	{
		let mut path = self.hyper_thread_folder_path(hyper_thread);
		path.push(file_name);
		path
	}
	
	/// A NUMA node file.
	#[inline(always)]
	pub fn numa_node_path(&self, numa_node: u8, file_name: &str) -> PathBuf
	{
		let mut path = self.numa_node_folder_path(numa_node);
		path.push(file_name);
		path
	}
	
	/// A PCI device file.
	#[inline(always)]
	pub fn pci_device_path(&self, pci_device: (u32, u8, u8, u8), file_name: &str) -> PathBuf
	{
		let mut path = self.pci_device_folder_path(pci_device);
		path.push(file_name);
		path
	}
	
	/// A path about all hyper threads.
	#[inline(always)]
	pub fn hyper_threads_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.hyper_threads_parent_path();
		path.push(file_name);
		path
	}
	
	/// A path about all NUMA nodes.
	#[inline(always)]
	pub fn numa_nodes_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.numa_nodes_parent_path();
		path.push(file_name);
		path
	}
	
	/// A path about all PCI devices.
	#[inline(always)]
	pub fn pci_devices_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.pci_devices_parent_path();
		path.push(file_name);
		path
	}
	
	/// Rescans all PCI buses and devices.
	///
	/// Errors are swallowed.
	#[inline(always)]
	pub fn rescan_all_pci_buses_and_devices(&self) -> io::Result<()>
	{
		let mut path = self.path();
		path.push("bus/pci/rescan");
		path.write_value(1)
	}
	
	#[inline(always)]
	pub(crate) fn workqueue_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/virtual/workqueue");
		path.push(file_name);
		path
	}
	
	/// Changes Transparent Huge Pages (THP) settings.
	///
	/// The value of the `transparent_huge_page_regular_memory_choice` can also be specified in the Linux kernel command line parameters as one of "transparent_hugepage=never", "transparent_hugepage=always" or "transparent_hugepage=madvise".
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn change_transparent_huge_pages_usage(&self, transparent_huge_page_regular_memory_choice: TransparentHugePageRegularMemoryChoice, transparent_huge_page_shared_memory_choice: TransparentHugePageSharedMemoryChoice, use_zero_page: bool) -> io::Result<()>
	{
		let use_zero_page_value = if use_zero_page
		{
			1
		}
		else
		{
			0
		};
		self.global_transparent_huge_memory_file_path("use_zero_page").write_value(use_zero_page_value)?;
		
		self.global_transparent_huge_memory_file_path("shmem_enabled").write_value(transparent_huge_page_shared_memory_choice.to_value())?;
		
		self.global_transparent_huge_memory_file_path("enabled").write_value(transparent_huge_page_regular_memory_choice.to_value())
	}
	
	/// Changes defragmentation using the Kernel-internal `khugepaged` daemon thread for Transparent Huge Pages (THP).
	///
	/// * The kernel default for `pages_to_scan` is 4096.
	/// * The kernel default for `scan_sleep_in_milliseconds` is 10_000.
	/// * The kernel default for `alloc_sleep_millisecs` is 60_000.
	/// * The kernel default for `how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages` is 511. Also known as `max_ptes_none`. A higher value leads to use additional memory for programs. A lower value produces less gains in performance. The value itself has very little effect on CPU usage.
	/// * The kernel default for `how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages` is 64. Also known as `max_ptes_swap`. A higher value can cause excessive swap IO and waste memory. A lower value can prevent THPs from being collapsed, resulting in fewer pages being collapsed into THPs, and so lower memory access performance.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn change_transparent_huge_pages_defragmentation(&self, transparent_huge_page_defragmentation_choice: TransparentHugePageDefragmentationChoice, pages_to_scan: u16, scan_sleep_in_milliseconds: usize, allocation_sleep_in_milliseconds: usize, how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages: u16, how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages: u16) -> io::Result<()>
	{
		self.khugepaged_file_path("pages_to_scan").write_value(pages_to_scan)?;
		self.khugepaged_file_path("alloc_sleep_millisecs").write_value(scan_sleep_in_milliseconds)?;
		self.khugepaged_file_path("scan_sleep_millisecs").write_value(allocation_sleep_in_milliseconds)?;
		self.khugepaged_file_path("max_ptes_none").write_value(how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages)?;
		self.khugepaged_file_path("max_ptes_swap").write_value(how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages)?;
		self.khugepaged_file_path("defrag").write_value(transparent_huge_page_defragmentation_choice.defrag_value())?;
		self.global_transparent_huge_memory_file_path("defrag").write_value(transparent_huge_page_defragmentation_choice.to_value())?;
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn read_global_hugepages_value(&self, huge_page_size: HugePageSize, file_name: &str) -> io::Result<u64>
	{
		self.global_hugepages_file_path(huge_page_size, file_name).read_value()
	}
	
	#[inline(always)]
	pub(crate) fn read_numa_hugepages_value(&self, huge_page_size: HugePageSize, numa_node: u8, file_name: &str) -> io::Result<u64>
	{
		self.numa_hugepages_file_path(huge_page_size, numa_node, file_name).read_value()
	}
	
	#[inline(always)]
	pub(crate) fn global_hugepages_file_path(&self, huge_page_size: HugePageSize, file_name: &str) -> PathBuf
	{
		let mut file_path = self.global_memory_folder_path();
		file_path.push(format!("hugepages/hugepages-{}kB", huge_page_size.size_in_kilo_bytes()));
		file_path.push(file_name);
		file_path
	}
	
	#[inline(always)]
	pub(crate) fn numa_hugepages_file_path(&self, huge_page_size: HugePageSize, numa_node: u8, file_name: &str) -> PathBuf
	{
		let mut file_path = self.numa_node_folder_path(numa_node);
		file_path.push(format!("hugepages/hugepages-{}kB", huge_page_size.size_in_kilo_bytes()));
		file_path.push(file_name);
		file_path
	}
	
	#[inline(always)]
	pub(crate) fn hyper_thread_folder_path(&self, hyper_thread: HyperThread) -> PathBuf
	{
		let into: u16 = hyper_thread.into();
		self.hyper_threads_path(&format!("cpu{}", into))
	}
	
	#[inline(always)]
	pub(crate) fn numa_node_folder_path(&self, numa_node: u8) -> PathBuf
	{
		self.numa_nodes_path(&format!("node{}", numa_node))
	}
	
	#[inline(always)]
	pub(crate) fn pci_device_folder_path(&self, pci_device: (u32, u8, u8, u8)) -> PathBuf
	{
		self.pci_devices_path(&format!("{:04x}:{:02x}:{:02x}.{:01x}", pci_device.0, pci_device.1, pci_device.2, pci_device.3))
	}
	
	#[inline(always)]
	fn hyper_threads_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/system/cpu");
		path
	}
	
	#[inline(always)]
	fn numa_nodes_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/system/node");
		path
	}
	
	#[inline(always)]
	fn pci_devices_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("bus/pci/devices");
		path
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn khugepaged_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.global_transparent_huge_memory_file_path("khugepaged");
		path.push(file_name);
		path
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn global_transparent_huge_memory_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.global_memory_folder_path();
		path.push("transparent_hugepage");
		path.push(file_name);
		path
	}
	
	#[inline(always)]
	fn global_memory_folder_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("kernel/mm");
		path
	}
	
	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
