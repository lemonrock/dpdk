// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


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
	/// Note that this might be a fake NUMA node, ie one lacking any CPUs.
	#[inline(always)]
	pub fn is_a_numa_node(&self, numa_node: u8) -> bool
	{
		self.numa_node_folder_path(numa_node).exists()
	}
	
	/// A CPU node file.
	#[inline(always)]
	pub fn cpu_node_path(&self, cpu_node: u16, file_name: &str) -> PathBuf
	{
		let mut path = self.cpu_node_folder_path(cpu_node);
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
	
	/// A path about all CPU nodes.
	#[inline(always)]
	pub fn cpu_nodes_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.cpu_nodes_parent_path();
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
	pub(crate) fn cpu_node_folder_path(&self, cpu_node: u16) -> PathBuf
	{
		self.cpu_nodes_path(&format!("cpu{}", cpu_node))
	}
	
	#[inline(always)]
	pub(crate) fn numa_node_folder_path(&self, numa_node: u8) -> PathBuf
	{
		self.numa_nodes_path(&format!("node{}", numa_node))
	}
	
	/// A path about all NUMA nodes.
	#[inline(always)]
	fn cpu_nodes_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/system/cpu");
		path
	}
	
	/// A path about all NUMA nodes.
	#[inline(always)]
	fn numa_nodes_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/system/node");
		path
	}
	
	#[inline(always)]
	pub(crate) fn global_memory_folder_path(&self) -> PathBuf
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
