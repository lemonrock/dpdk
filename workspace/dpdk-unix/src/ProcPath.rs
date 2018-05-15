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
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn global_zoned_virtual_memory_statistics(&self) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		self.file_path("vmstat").parse_virtual_memory_statistics_file()
	}
	
	/// Memory information (from `/proc/meminfo`).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn memory_information(&self, memory_information_name_prefix: &str) -> Result<MemoryInformation, MemoryInformationParseError>
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
	
	/// Returns known interrupt requests (IRQs).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn interrupt_requests(&self) -> Result<BTreeSet<u16>, io::Error>
	{
		let mut interrupt_requests = BTreeSet::new();
		
		let irq_folder_path = self.file_path("irq");
		for entry in irq_folder_path.read_dir()?
		{
			let entry = entry?;
			if entry.file_type()?.is_file()
			{
				if let Ok(string) = entry.file_name().as_os_str().to_str()
				{
					if let Ok(interrupt_request) = string.parse::<u16>()
					{
						interrupt_requests.push(interrupt_request)
					}
				}
			}
		}
		
		Ok(interrupt_requests)
	}
	
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_interrupt_request_to_hyper_threads_affinity(&self, interrupt_request: u16) -> Result<BTreeSet<u16>, io::Error>
	{
		self.file_path(&format!("irq/{}/smp_affinity_list", interrupt_request)).read_linux_core_or_numa_list()
	}
	
	/// This logic seems to exist in the Linux kernel to provide a place for the `irqbalance` daemon to store some configuration.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_interrupt_request_to_hyper_threads_affinity_hint(&self, interrupt_request: u16) -> Result<BTreeSet<u16>, io::Error>
	{
		self.file_path(&format!("irq/{}/affinity_hint", interrupt_request)).read_linux_core_or_numa_list()
	}
	
	/// ?numa node? As always, the Linux documentation sucks.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_interrupt_request_node(&self, interrupt_request: u16) -> Result<u8, io::Error>
	{
		self.file_path(&format!("irq/{}/node", interrupt_request)).read_value()
	}
	
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_default_interrupt_request_affinity_cpu_mask(&self) -> Result<u32, io::Error>
	{
		self.file_path("irq/default_smp_affinity").parse_linux_core_or_numa_mask()
	}
	
	/// We ignore failures as the `/proc` for interrupt requests is brittle.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn force_all_interrupt_requests_to_just_these_hyper_threads(&self, hyper_threads: &BTreeSet<u16>)
	{
		let mask = Self::hyper_threads_to_mask(hyper_threads);
		
		self.file_path("irq/default_smp_affinity").write_value(&mask);
		
		if let Some(interrupt_requests) = self.interrupt_requests()
		{
			for interrupt_request in interrupt_requests.iter()
			{
				self.file_path(&format!("irq/{}/smp_affinity", interrupt_request)).write_value(&mask);
				self.file_path(&format!("irq/{}/affinity_hint", interrupt_request)).write_value(&mask);
			}
		}
	}
	
	/// We ignore failures as the `/proc` for this is brittle.
	///
	/// Should not be needed if `nohz_full` was specified on the Linux command line.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn force_watchdog_to_just_these_hyper_threads(&self, hyper_threads: &BTreeSet<u16>)
	{
		let mut list = String::with_capacity(hyper_threads.len() * 4);
		for hyper_thread in hyper_threads.iter()
		{
			if !list.is_empty()
			{
				list.push(',');
			}
			list.push_str(&format!("{}", hyper_thread))
		}
		
		self.file_path("sys/kernel/watchdog_cpumask").write_value(&list);
	}
	
	#[inline(always)]
	pub(crate) fn hyper_threads_to_mask(hyper_threads: &BTreeSet<u16>) -> String
	{
		let mut mask: u32 = 0;
		for hyper_thread in hyper_threads.iter()
		{
			let bit = (1 << hyper_thread) as u32;
			mask |= bit;
		}
		format!("{:08x}", mask)
	}
	
	/// Only execute this afte any kernel modules have loaded.
	///
	/// We ignore failures.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn write_system_control_values(&self, settings: HashMap<String, u64>)
	{
		for (setting_name, setting_value) in settings.iter()
		{
			let file_path = self.file_path(&format!("sys/{}", setting_name));
			file_path.write_value(setting_value);
		}
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
