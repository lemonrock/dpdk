// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryStatistics(HashMap<MemoryStatisticName, u64>);

// Super-detailed information (hard to parse, too) is in /proc/zoneinfo
// This is broken down into DMA, DMA33 and Normal sub-zones and then by CPU for each Numa Node (aka 'zone')
// A sort of detailed version of /proc/vmstat
impl MemoryStatistics
{
	#[inline]
	pub fn getStatistic(&self, memoryStatisticName: &MemoryStatisticName) -> Option<u64>
	{
		match self.0.get(memoryStatisticName)
		{
			None => None,
			Some(value) => Some(*value),
		}
	}
	
	pub fn freePhysicalRam(&self) -> Option<u64>
	{
		self.getStatistic(&MemoryStatisticName::FreePhysicalRam)
	}
	
	pub fn defaultHugePageSize(&self) -> Option<HugePageSize>
	{
		if let Some(sizeInBytes) = self.getStatistic(&MemoryStatisticName::SizeOfAHugePage)
		{
			HugePageSize::fromProcMemInfoValue(sizeInBytes)
		}
		else
		{
			None
		}
	}
	
	#[inline]
	pub fn usedPhysicalRam(&self) -> Option<u64>
	{
		if let Some(totalPhysicalRam) = self.getStatistic(&MemoryStatisticName::TotalPhyiscalRam)
		{
			if let Some(freePhysicalRam) = self.getStatistic(&MemoryStatisticName::FreePhysicalRam)
			{
				Some(totalPhysicalRam - freePhysicalRam)
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
	
	#[inline]
	pub fn usedSwap(&self) -> Option<u64>
	{
		if let Some(totalSwap) = self.getStatistic(&MemoryStatisticName::TotalSwap)
		{
			if let Some(freeSwap) = self.getStatistic(&MemoryStatisticName::FreeSwap)
			{
				Some(totalSwap - freeSwap)
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
	
	pub fn forMachine(procPath: &Path) -> Result<Self, MemoryStatisticsParseError>
	{
		Self::parse(procPath, "")
	}
	
	pub fn parse(parentFolderPath: &Path, memoryStatisticNamePrefix: &str) -> Result<Self, MemoryStatisticsParseError>
	{
		let mut meminfoFilePath = PathBuf::from(parentFolderPath);
		meminfoFilePath.push("meminfo");
		
		let openFile = try!(File::open(meminfoFilePath));
		let mut reader = BufReader::with_capacity(4096, openFile);
		
		let mut memoryStatistics = HashMap::new();
		let mut lineCount = 0;
		let mut line = String::with_capacity(512);
		while try!(reader.read_line(&mut line)) > 0
		{
			{
				let mut split = line.splitn(2, ':');

				let memoryStatisticName = MemoryStatisticName::parse(split.next().unwrap(), memoryStatisticNamePrefix);
				
				let memoryStatisticValue = match split.next()
				{
					None => return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValue(lineCount, memoryStatisticName)),
					Some(rawValue) =>
					{
						let trimmedRawValue = rawValue.trim();
						let endsWith = memoryStatisticName.unit().endsWith();
					
						if !trimmedRawValue.ends_with(endsWith)
						{
							return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValue(lineCount, memoryStatisticName));
						}
					
						let trimmed = &rawValue[0..rawValue.len() - endsWith.len()];
					
						match trimmed.parse::<u64>()
						{
							Ok(value) => value,
							Err(intParseError) => return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValueAsU64(lineCount, memoryStatisticName, rawValue.to_owned(), intParseError))
						}
					}
				};
				
				if memoryStatistics.contains_key(&memoryStatisticName)
				{
					return Err(MemoryStatisticsParseError::DuplicateMemoryStatistic(lineCount, memoryStatisticName, memoryStatisticValue));
				}
				
				memoryStatistics.insert(memoryStatisticName, memoryStatisticValue);
			}
			
			line.clear();
			lineCount += 1;
		}
		
		Ok(MemoryStatistics(memoryStatistics))
	}
}
