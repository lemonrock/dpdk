// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


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
	/// Is autogroup active? (from `/proc/sys/kernel/sched_autogroup_enabled`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn is_autogroup_active(&self) -> Result<bool, io::Error>
	{
		let value = self.sched_autogroup_enabled_file_path().read_raw_without_line_feed()?;
		match &value[..]
		{
			b"0" => Ok(false),
			b"1" => Ok(true),
			_ => Err(io::Error::from(ErrorKind::InvalidData)),
		}
	}

	/// Enable the autogroup feature (requires Root).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn enable_autogroup(&self) -> Result<(), io::Error>
	{
		self.sched_autogroup_enabled_file_path().write_value("1")
	}

	/// Disable the autogroup feature (requires Root).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn disable_autogroup(&self) -> Result<(), io::Error>
	{
		self.sched_autogroup_enabled_file_path().write_value("0")
	}

	/// Adjust the autogroup setting of nice for the current process.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn adjust_autogroup_nice_value_for_self(&self, nice_value: Nice) -> Result<(), io::Error>
	{
		self.file_path("self/autogroup").write_value(nice_value)
	}

	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	fn sched_autogroup_enabled_file_path(&self) -> PathBuf
	{
		self.file_path("sys/kernel/sched_autogroup_enabled")
	}

	/// Status information from `/proc/self/status`.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn self_status(&self) -> Result<ProcessStatusStatistics, ProcessStatusFileParseError>
	{
		self.file_path("self/status").parse_process_status_file()
	}

	/// Status information from `/proc/<IDENTIFIER>/status` where `<IDENTIFIER>` is `identifier`.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn process_status(&self, identifier: pid_t) -> Result<ProcessStatusStatistics, ProcessStatusFileParseError>
	{
		self.file_path(&format!("{}/status", identifier)).parse_process_status_file()
	}

	/// Memory statistics (from `/proc/vmstat`).
	///
	/// Interpret this by multiplying counts by page size.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn global_zoned_virtual_memory_statistics(&self) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		self.file_path("vmstat").parse_virtual_memory_statistics_file()
	}

	/// Memory information (from `/proc/meminfo`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn memory_information(&self, memory_information_name_prefix: &[u8]) -> Result<MemoryInformation, MemoryInformationParseError>
	{
		self.file_path("meminfo").parse_memory_information_file(memory_information_name_prefix)
	}
	
	/// File systems (from `/proc/filesystems`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn filesystems(&self) -> Result<FileSystemTypeList, io::Error>
	{
		let file_path = self.file_path("filesystems");
		FileSystemTypeList::parse(&file_path)
	}
	
	/// Current mounts (from `/proc/self/mounts`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn mounts(&self) -> Result<Mounts, io::Error>
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
	
	/// Command line parameters used to start Linux.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn linux_command_line_parameters(&self) -> Result<LinuxKernelCommandLineParameters, io::Error>
	{
		let file_path = self.file_path("cmdline");
		LinuxKernelCommandLineParameters::parse(&file_path)
	}
	
	/// Only execute this afte any kernel modules have loaded.
	///
	/// We ignore failures.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn write_system_control_values(&self, settings: &HashMap<String, u64>) -> io::Result<()>
	{
		for (setting_name, setting_value) in settings.iter()
		{
			let file_path = self.file_path(&format!("sys/{}", setting_name));
			file_path.write_value(setting_value)?;
		}
		Ok(())
	}
	
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
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
