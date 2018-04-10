
// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const MaximumNumaSockets: usize = RTE_MAX_NUMA_NODES;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct NumaSocketId(u8);

impl AnyNumaSocketId for NumaSocketId
{
	#[inline(always)]
	fn isAny(&self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn as_c_int(&self) -> c_int
	{
		self.0 as c_int
	}
	
	#[inline(always)]
	fn as_c_uint(&self) -> c_uint
	{
		self.0 as u32
	}
	
	#[inline(always)]
	fn as_int32_t(&self) -> int32_t
	{
		self.0 as int32_t
	}
}

#[inline(always)]
fn typeOfMemoryX(typeOfMemory: Option<ConstCStr>) -> *const c_char
{
	if typeOfMemory.is_none()
	{
		null()
	}
	else
	{
		typeOfMemory.unwrap().as_ptr()
	}
}

impl NumaSocketId
{
	pub const SocketZeroAlwaysExists: NumaSocketId = NumaSocketId(0);
	pub const Any: Option<NumaSocketId> = None;
	
	#[inline(always)]
	pub fn numaNodesData(sysPath: &Path) -> Result<Option<NumaNodesData>, ListParseError>
	{
		// Check is this a NUMA machine (ie kernel has CONFIG_NUMA = y)
		let nodesPath = Self::nodesSysPath(sysPath);
		if !nodesPath.is_dir()
		{
			return Ok(None);
		}
		
		fn parse(nodesPath: &Path, item: &str) -> Result<NumaSocketsActive, ListParseError>
		{
			let mut nodesItemPath = PathBuf::from(nodesPath);
			nodesItemPath.push(item);
			NumaSocketsActive::parseFromFilePath(&nodesItemPath)
		}
		
		let nodesPath = &nodesPath;
		
		Ok
		(
			Some
			(
				NumaNodesData
				{
					hasCpu: parse(nodesPath, "has_cpu")?,
					hasMemory: parse(nodesPath, "has_memory")?,
					hasNormalMemory: parse(nodesPath, "has_normal_memory")?,
					online: parse(nodesPath, "online")?,
					possible: parse(nodesPath, "possible")?,
					// There also was 'has_high_memory' at one time (CONFIG_HIGHMEM)
				}
			)
		)
	}

	/// Do not use this; everything here and more is in virtualMemoryPageUsageStatistics()
	/// Interpret this by multiplying counts by small PageSize
	pub fn numaStatistics(&self, sysPath: &Path) -> io::Result<NumaNodeStatistics>
	{
		self.statisticListParse(sysPath, "numastat")
	}
	
	/// Interpret this by multiplying counts by small PageSize
	pub fn virtualMemoryPageUsageStatistics(&self, sysPath: &Path) -> io::Result<NumaNodeStatistics>
	{
		self.statisticListParse(sysPath, "vmstat")
	}

	/// Similar to virtualMemoryPageUsageStatistics() except (a) values are sized in KiloBytes and (b) values take into account huge pages (we think)
	/// Also contains MemoryStatisticName::TotalPhysicalRam and MemoryStatisticName::FreePhysicalRam
	pub fn meminfo(&self, sysPath: &Path) -> Result<MemoryStatistics, MemoryStatisticsParseError>
	{
		let folderPath = self.nodeSysPath(sysPath);
		MemoryStatistics::parse(&folderPath, &format!("Node {} ", self.0))
	}
	
	pub fn cpuList(&self, sysPath: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		let filePath = self.itemPath(sysPath, "cpulist");
		LogicalCoresActive::parseFromFilePath(&filePath)
	}
	
	pub fn distance(&self, sysPath: &Path) -> io::Result<u8>
	{
		let filePath = self.itemPath(sysPath, "distance");
		filePath.read_value()
	}
	
	pub fn numberOfHugePages(&self, sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = self.numberOfHugePagesFilePath(sysPath, hugePageSize);
		filePath.read_value()
	}
	
	pub fn numberOfFreeHugePages(&self, sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = self.hugepagesPath(sysPath, hugePageSize, "free_hugepages");
		filePath.read_value()
	}
	
	pub fn numberOfSurplusHugePages(&self, sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = self.hugepagesPath(sysPath, hugePageSize, "surplus_hugepages");
		filePath.read_value()
	}
	
	/// Will only work as root
	pub fn tryToCompact(&self, sysPath: &Path) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Compact NUMA node '{}'", self.0));

		let path = self.itemPath(sysPath, "compact");
		path.write_value(1)
	}
	
	/// Will only work as root
	pub fn tryToEvictPages(&self, sysPath: &Path) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Evict pages on NUMA node '{}'", self.0));

		let path = self.itemPath(sysPath, "scan_unevictable_pages");
		path.write_value(1)
	}
	
	/// Will only work as root
	pub fn tryToClearAllHugePagesReserved(&self, sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Clear all huge pages of size '{:?}' reserved on NUMA node '{}'", hugePageSize, self.0));
		self.tryToReserveHugePages(sysPath, hugePageSize, 0)
	}
	
	/// Will only work as root
	pub fn tryToReserveHugePages(&self, sysPath: &Path, hugePageSize: HugePageSize, count: u64) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Reserve '{}' huge pages of size '{:?}' reserved on NUMA node '{}'", count, hugePageSize, self.0));

		let filePath = self.numberOfHugePagesFilePath(sysPath, hugePageSize);
		filePath.write_value(count)
	}
	
	fn statisticListParse(&self, sysPath: &Path, statisticsFileName: &str) -> io::Result<NumaNodeStatistics>
	{
		let path = self.itemPath(sysPath, statisticsFileName);
		let openFile = File::open(path)?;

		let mut reader = BufReader::with_capacity(4096, openFile);

		let mut numaNodeStatistics = NumaNodeStatistics::with_capacity(6);
		let mut lineNumber = 0;
		let mut line = String::with_capacity(64);
		while reader.read_line(&mut line)? > 0
		{
			{
				let mut split = line.splitn(2, ' ');
				
				let numaNodeStatisticName = NumaNodeStatisticName::parse(split.next().unwrap());
				
				let statisticValue = match split.next()
				{
					None => return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero based line '{}' does not have a value second column", lineNumber))),
					Some(value) =>
					{
						match value.parse::<u64>()
						{
							Err(parseError) => return Err(io::Error::new(ErrorKind::InvalidData, parseError)),
							Ok(value) => value,
						}
					}
				};
				
				if let Some(previous) = numaNodeStatistics.insert(numaNodeStatisticName, statisticValue)
				{
					return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero based line '{}' has a duplicate statistic (was '{}')", lineNumber, previous)))
				}
			}
			
			line.clear();
			lineNumber += 1;
		}
		
		Ok(numaNodeStatistics)
	}
	
	fn numberOfHugePagesFilePath(&self, sysPath: &Path, hugePageSize: HugePageSize) -> PathBuf
	{
		self.hugepagesPath(sysPath, hugePageSize, "nr_hugepages")
	}
	
	#[inline(always)]
	fn nodesSysPath(sysPath: &Path) -> PathBuf
	{
		let mut nodesSysPath = PathBuf::from(sysPath);
		nodesSysPath.push("devices/system/node");
		nodesSysPath
	}
	
	#[inline(always)]
	fn nodesItemSysPath(sysPath: &Path, item: &str) -> PathBuf
	{
		let mut nodesItemSysPath = Self::nodesSysPath(sysPath);
		nodesItemSysPath.push(item);
		nodesItemSysPath
	}
	
	#[inline(always)]
	fn nodeSysPath(&self, sysPath: &Path) -> PathBuf
	{
		Self::nodesItemSysPath(sysPath, &format!("node{}", self.0))
	}
	
	#[inline(always)]
	fn itemPath(&self, sysPath: &Path, item: &str) -> PathBuf
	{
		let mut path = self.nodeSysPath(sysPath);
		path.push(item);
		path
	}
	
	#[inline(always)]
	fn hugepagesPath(&self, sysPath: &Path, hugePageSize: HugePageSize, item: &str) -> PathBuf
	{
		self.itemPath(sysPath, &format!("hugepages/hugepages-{}kB/{}", hugePageSize.size(), item.to_owned()))
	}
	
	#[inline(always)]
	pub fn findInstalledNumberOfNumaSockets() -> usize
	{
		let mut currentMaximum = NumaSocketId::SocketZeroAlwaysExists;
		for logicalCoreIdentifier in 0..(MaximumLogicalCores as u32)
		{
			if let Some(numaSocketId) = LogicalCore(logicalCoreIdentifier).optionalNumaSocketId()
			{
				if numaSocketId > currentMaximum
				{
					currentMaximum = numaSocketId;
				}
			}
		}
		
		(currentMaximum.0 + 1) as usize
	}
	
	#[inline(always)]
	pub fn choose<'a, V>(&'a self, from: &'a [V; MaximumNumaSockets]) -> &V
	{
		&from[self.0 as usize]
	}
	
	#[inline(always)]
	pub fn getSocketStatistics(&self) -> rte_malloc_socket_stats
	{
		let mut statistics = unsafe { uninitialized() };
		let result = unsafe { ::dpdk_sys::rte_malloc_get_socket_stats(self.as_c_int(), &mut statistics) };
		if likely(result == 0)
		{
			statistics
		}
		else
		{
			forget(statistics);
			
			panic!("rte_malloc_get_socket_stats() returned '{}'", result);
		}
	}
	
	#[inline(always)]
	pub fn dumpAllocationStatisticsToStandardError(typeOfMemory: Option<ConstCStr>)
	{
		unsafe { ::dpdk_sys::rte_malloc_dump_stats(stderr as *mut FILE, typeOfMemoryX(typeOfMemory)) }
	}
	
	#[inline(always)]
	pub fn forCurrentLogicalCore() -> Option<NumaSocketId>
	{
		Self::fromU32(unsafe { ::dpdk_sys::rte_socket_id() })
	}
	
	#[inline(always)]
	pub fn fromI32(value: i32) -> Option<NumaSocketId>
	{
		if unlikely(value == SOCKET_ID_ANY)
		{
			None
		}
		else
		{
			Self::fromU32(value as u32)
		}
	}
	
	#[inline(always)]
	pub fn fromU32(value: u32) -> Option<NumaSocketId>
	{
		debug_assert!(value <= MaximumNumaSockets as u32, "value '{}' is equal to or exceeds MaximumNumaSockets, '{}'", value, MaximumNumaSockets);
		Some(NumaSocketId(value as u8))
	}
	
	#[inline(always)]
	pub fn as_u8(&self) -> u8
	{
		self.0
	}
	
	#[inline(always)]
	pub fn as_usize(&self) -> usize
	{
		self.0 as usize
	}
}
