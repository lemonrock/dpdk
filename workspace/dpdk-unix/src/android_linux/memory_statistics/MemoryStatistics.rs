// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A set of memory statistics.
///
/// Super-detailed information (hard to parse, too) is in `/proc/zoneinfo`.
/// This is broken down into DMA, DMA33 and Normal sub-zones and then by CPU for each Numa Node (aka 'zone').
/// A sort of detailed version of `/proc/vmstat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryStatistics(HashMap<MemoryStatisticName, u64>);

impl MemoryStatistics
{
	/// Get a statistic.
	#[inline]
	pub fn get_statistic(&self, memory_statistic_name: &MemoryStatisticName) -> Option<u64>
	{
		match self.0.get(memory_statistic_name)
		{
			None => None,
			Some(value) => Some(*value),
		}
	}
	
	/// Free physical RAM in bytes.
	pub fn free_physical_ram(&self) -> Option<u64>
	{
		self.get_statistic(&MemoryStatisticName::FreePhysicalRam)
	}
	
	/// Default huge page size.
	pub fn default_huge_page_size(&self) -> Option<HugePageSize>
	{
		if let Some(size_in_bytes) = self.get_statistic(&MemoryStatisticName::SizeOfAHugePage)
		{
			HugePageSize::from_proc_mem_info_value(size_in_bytes)
		}
		else
		{
			None
		}
	}
	
	/// Used physical RAM in bytes.
	#[inline]
	pub fn used_physical_ram(&self) -> Option<u64>
	{
		if let Some(total_physical_ram) = self.get_statistic(&MemoryStatisticName::TotalPhysicalRam)
		{
			if let Some(free_physical_ram) = self.get_statistic(&MemoryStatisticName::FreePhysicalRam)
			{
				Some(total_physical_ram - free_physical_ram)
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
	
	/// Used swap RAM in bytes.
	#[inline]
	pub fn used_swap(&self) -> Option<u64>
	{
		if let Some(total_swap) = self.get_statistic(&MemoryStatisticName::TotalSwap)
		{
			if let Some(free_swap) = self.get_statistic(&MemoryStatisticName::FreeSwap)
			{
				Some(total_swap - free_swap)
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
	
	/// Parses the `meminfo` file in the given `proc_path` for the machine.
	pub fn parse_for_machine(proc_path: &Path) -> Result<Self, MemoryStatisticsParseError>
	{
		Self::parse(proc_path, "")
	}
	
	/// Parses the `meminfo` file in the given `parent_folder_path`.
	pub fn parse(parent_folder_path: &Path, memory_statistic_name_prefix: &str) -> Result<Self, MemoryStatisticsParseError>
	{
		let mut meminfo_file_path = PathBuf::from(parent_folder_path);
		meminfo_file_path.push("meminfo");
		
		let mut reader = BufReader::with_capacity(4096, File::open(meminfo_file_path)?);
		
		let mut memory_statistics = HashMap::new();
		let mut line_number = 0;
		let mut line = String::with_capacity(512);
		while reader.read_line(&mut line)? > 0
		{
			{
				let mut split = line.splitn(2, ':');

				let memory_statistic_name = MemoryStatisticName::parse(split.next().unwrap(), memory_statistic_name_prefix);
				
				let memory_statistic_value = match split.next()
				{
					None => return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValue(line_number, memory_statistic_name)),
					Some(raw_value) =>
					{
						let trimmed_raw_value = raw_value.trim();
						let ends_with = memory_statistic_name.unit().ends_with();
					
						if !trimmed_raw_value.ends_with(ends_with)
						{
							return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValue(line_number, memory_statistic_name));
						}
					
						let trimmed = &raw_value[0..raw_value.len() - ends_with.len()];
					
						match trimmed.parse::<u64>()
						{
							Ok(value) => value,
							Err(int_parse_error) => return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValueAsU64(line_number, memory_statistic_name, raw_value.to_owned(), int_parse_error))
						}
					}
				};
				
				if memory_statistics.contains_key(&memory_statistic_name)
				{
					return Err(MemoryStatisticsParseError::DuplicateMemoryStatistic(line_number, memory_statistic_name, memory_statistic_value));
				}
				
				memory_statistics.insert(memory_statistic_name, memory_statistic_value);
			}
			
			line.clear();
			line_number += 1;
		}
		
		Ok(MemoryStatistics(memory_statistics))
	}
}
