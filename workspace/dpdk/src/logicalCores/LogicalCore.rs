// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl LogicalCore
{
	pub fn online(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "online")
	}

	/// Not useful, as includes cpus that can never be brought online (see possible)
	pub fn offline(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "offline")
	}

	/// Not reliable, as includes cpus that can never be brought online; simply reports CPUs that could be used by the Kernel upto the CONFIG_? number of CPUs
	pub fn possible(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "possible")
	}

	/// Not reliable, as includes cpus that can never be brought online; simply reports CPUs that could be used by the Kernel upto the CONFIG_? number of CPUs
	pub fn present(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "present")
	}

	pub fn kernelMaximumCpuIndex(sys_path: &Path) -> io::Result<u32>
	{
		let file_path = Self::cpusItemSysPath(sys_path, "kernel_max");
		file_path.read_value()
	}

	fn cpulist(sys_path: &Path, fileName: &str) -> Result<LogicalCoresActive, ListParseError>
	{
		let file_path = Self::cpusItemSysPath(sys_path, fileName);
		LogicalCoresActive::parse_from_file_path(&file_path)
	}

	pub fn topologyCoreId(&self, sys_path: &Path) -> io::Result<u64>
	{
		let file_path = self.topologyFilePath(sys_path, "core_id");
		file_path.read_value()
	}

	#[inline(always)]
	fn cpusSysPath(sys_path: &Path) -> PathBuf
	{
		let mut nodesSysPath = PathBuf::from(sys_path);
		nodesSysPath.push("devices/system/cpu");
		nodesSysPath
	}

	#[inline(always)]
	fn cpusItemSysPath(sys_path: &Path, item: &str) -> PathBuf
	{
		let mut nodesItemSysPath = Self::cpusSysPath(sys_path);
		nodesItemSysPath.push(item);
		nodesItemSysPath
	}

	#[inline(always)]
	fn cpuSysPath(&self, sys_path: &Path) -> PathBuf
	{
		if self.isAny()
		{
			panic!("Any logical core does not have a cpuSysPath");
		}

		Self::cpusItemSysPath(sys_path, &format!("cpu{}", self.0))
	}

	#[inline(always)]
	fn topologyFilePath(&self, sys_path: &Path, fileName: &str) -> PathBuf
	{
		let mut path = self.cpuSysPath(sys_path);
		path.push("topology");
		path.push(fileName);
		path
	}
}
