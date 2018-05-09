// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents `/proc`.
///
/// Frankly, there are files in `/proc` that really belong in `/sys`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ProcPath(PathBuf);

impl Default for ProcPath
{
	#[inline(always)]
	fn default() -> Self
	{
		ProcPath(PathBuf::from("/proc"))
	}
}

impl ProcPath
{
	/// Memory statistics (from `/proc/vmstat`).
	///
	/// Interpret this by multiplying counts by page size.
	#[inline(always)]
	pub fn global_zoned_virtual_memory_statistics(&self) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		self.file_path("vmstat").parse_virtual_memory_statistics_file()
	}
	
	/// Memory information (from `/proc/meminfo`).
	#[inline(always)]
	pub fn memory_statistics(&self, memory_statistic_name_prefix: &str) -> Result<MemoryStatistics, MemoryStatisticsParseError>
	{
		self.file_path("meminfo").parse_memory_information_file(memory_statistic_name_prefix)
	}
	
	/// File systems (from `/proc/filesystems`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn filesystems(&self) -> Result<HashMap<FileSystemType, HasNoAssociatedDevice>, io::Error>
	{
		let file_path = self.file_path("filesystems");
		FileSystemType::parse(&file_path)
	}
	
	/// Current mounts (from `/proc/self/mounts`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn mounts(&self) -> Result<HashMap<PathBuf, Mount>, io::Error>
	{
		let file_path = self.file_path("self/mounts");
		Mounts::parse(&file_path)
	}
	
	/// Current loaded Linux kernel modules (from `/proc/modules`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn modules(&self) -> Result<LinuxKernelModulesList, LinuxKernelModulesListParseError>
	{
		let file_path = self.file_path("modules");
		LinuxKernelModulesList::parse(&file_path)
	}
	
	#[inline(always)]
	pub(crate) fn maximum_number_of_open_file_descriptors(&self) -> io::Result<u64>
	{
		self.file_path("sys/fs/nr_open").read_value()
	}
	
	#[inline(always)]
	fn file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.path();
		path.push(file_name);
		path
	}
	
	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
