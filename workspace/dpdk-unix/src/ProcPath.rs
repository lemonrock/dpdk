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
		let mut path = self.path();
		path.push("vmstat");
		path.parse_virtual_memory_statistics_file()
	}
	
	/// `/proc/meminfo`.
	#[inline(always)]
	pub fn memory_statistics(&self, memory_statistic_name_prefix: &str) -> Result<MemoryStatistics, MemoryStatisticsParseError>
	{
		let mut path = self.path();
		path.push("meminfo");
		path.parse_memory_information_file(memory_statistic_name_prefix)
	}
	
	/// `/proc/modules`.
	#[inline(always)]
	pub fn modules(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("modules");
		path
	}
	
	/// Obtains the maximum number of file descriptors as a finite resource limit.
	#[inline(always)]
	pub(crate) fn maximum_number_of_open_file_descriptors(&self) -> io::Result<u64>
	{
		let mut nr_open_file_path = self.path();
		nr_open_file_path.push("sys/fs/nr_open");
		nr_open_file_path.read_value()
	}
	
	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
