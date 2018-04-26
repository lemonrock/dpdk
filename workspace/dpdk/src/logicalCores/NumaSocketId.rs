
// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



pub trait AnyNumaSocketId
{
	#[inline(always)]
	fn isAny(&self) -> bool;
	
	#[inline(always)]
	fn as_c_int(&self) -> c_int;
	
	#[inline(always)]
	fn as_c_uint(&self) -> c_uint;
	
	#[inline(always)]
	fn as_int32_t(&self) -> int32_t;
	
	const CacheLineSize: u32 = 64;
	
	#[inline(always)]
	fn allocate<T>(&self, typeOfMemory: Option<ConstCStr>, size: usize, alignment: Option<PowerOfTwoThirtyTwoBit>) -> Option<DpdkAllocatedMemory<T>>
	{
		let alignment = alignment.as_u32();
		debug_assert!(alignment == 0 || alignment >= Self::CacheLineSize, "alignment must be greater than or equal to cache line size '{}', not '{}'", Self::CacheLineSize, alignment);
		
		let result = unsafe { rte_malloc_socket(typeOfMemoryX(typeOfMemory), size, alignment, self.as_c_int()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}
	
	#[inline(always)]
	fn zeroAllocate<T>(&self, typeOfMemory: Option<ConstCStr>, size: usize, alignment: Option<PowerOfTwoThirtyTwoBit>) -> Option<DpdkAllocatedMemory<T>>
	{
		let alignment = alignment.as_u32();
		debug_assert!(alignment == 0 || alignment >= Self::CacheLineSize, "alignment must be greater than or equal to cache line size '{}', not '{}'", Self::CacheLineSize, alignment);
		
		let result = unsafe { rte_zmalloc_socket(typeOfMemoryX(typeOfMemory), size, alignment, self.as_c_int()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}
	
	#[inline(always)]
	fn cAllocate<T>(&self, typeOfMemory: Option<ConstCStr>, numberOfElements: usize, sizeOfAnElement: usize, alignment: Option<PowerOfTwoThirtyTwoBit>) -> Option<DpdkAllocatedMemory<T>>
	{
		let alignment = alignment.as_u32();
		debug_assert!(alignment == 0 || alignment >= Self::CacheLineSize, "alignment must be greater than or equal to cache line size '{}', not '{}'", Self::CacheLineSize, alignment);
		
		let result = unsafe { rte_calloc_socket(typeOfMemoryX(typeOfMemory), numberOfElements, sizeOfAnElement, alignment, self.as_c_int()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}
}

impl AnyNumaSocketId for Option<NumaSocketId>
{
	#[inline(always)]
	fn isAny(&self) -> bool
	{
		true
	}
	
	#[inline(always)]
	fn as_c_int(&self) -> c_int
	{
		SOCKET_ID_ANY as c_int
	}
	
	// Weird
	#[inline(always)]
	fn as_c_uint(&self) -> c_uint
	{
		0xFFFF_FFFF
	}
	
	#[inline(always)]
	fn as_int32_t(&self) -> int32_t
	{
		SOCKET_ID_ANY as int32_t
	}
}



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

impl NumaSocketId
{
	pub const SocketZeroAlwaysExists: NumaSocketId = NumaSocketId(0);
	pub const Any: Option<NumaSocketId> = None;

	#[inline(always)]
	pub fn numaNodesData(sys_path: &Path) -> Result<Option<NumaNodesData>, ListParseError>
	{
		// Check is this a NUMA machine (ie kernel has CONFIG_NUMA = y)
		let nodesPath = Self::nodesSysPath(sys_path);
		if !nodesPath.is_dir()
		{
			return Ok(None);
		}

		fn parse(nodesPath: &Path, item: &str) -> Result<NumaSocketsActive, ListParseError>
		{
			let mut nodesItemPath = PathBuf::from(nodesPath);
			nodesItemPath.push(item);
			NumaSocketsActive::parse_from_file_path(&nodesItemPath)
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
	pub fn numaStatistics(&self, sys_path: &Path) -> io::Result<NumaNodeStatistics>
	{
		self.statisticListParse(sys_path, "numastat")
	}

	/// Interpret this by multiplying counts by small PageSize
	pub fn virtualMemoryPageUsageStatistics(&self, sys_path: &Path) -> io::Result<NumaNodeStatistics>
	{
		self.statisticListParse(sys_path, "vmstat")
	}

	/// Similar to virtualMemoryPageUsageStatistics() except (a) values are sized in KiloBytes and (b) values take into account huge pages (we think)
	/// Also contains MemoryStatisticName::TotalPhysicalRam and MemoryStatisticName::FreePhysicalRam
	pub fn meminfo(&self, sys_path: &Path) -> Result<MemoryStatistics, MemoryStatisticsParseError>
	{
		let folderPath = self.nodeSysPath(sys_path);
		MemoryStatistics::parse(&folderPath, &format!("Node {} ", self.0))
	}

	pub fn cpuList(&self, sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		let file_path = self.itemPath(sys_path, "cpulist");
		LogicalCoresActive::parse_from_file_path(&file_path)
	}

	pub fn distance(&self, sys_path: &Path) -> io::Result<u8>
	{
		let file_path = self.itemPath(sys_path, "distance");
		file_path.read_value()
	}

	pub fn numberOfHugePages(&self, sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = self.numberOfHugePagesFilePath(sys_path, hugePageSize);
		file_path.read_value()
	}

	pub fn numberOfFreeHugePages(&self, sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = self.hugepagesPath(sys_path, hugePageSize, "free_hugepages");
		file_path.read_value()
	}

	pub fn numberOfSurplusHugePages(&self, sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = self.hugepagesPath(sys_path, hugePageSize, "surplus_hugepages");
		file_path.read_value()
	}

	/// Will only work as root
	pub fn tryToCompact(&self, sys_path: &Path) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Compact NUMA node '{}'", self.0));

		let path = self.itemPath(sys_path, "compact");
		path.write_value(1)
	}

	/// Will only work as root
	pub fn tryToEvictPages(&self, sys_path: &Path) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Evict pages on NUMA node '{}'", self.0));

		let path = self.itemPath(sys_path, "scan_unevictable_pages");
		path.write_value(1)
	}

	/// Will only work as root
	pub fn tryToClearAllHugePagesReserved(&self, sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Clear all huge pages of size '{:?}' reserved on NUMA node '{}'", hugePageSize, self.0));
		self.tryToReserveHugePages(sys_path, hugePageSize, 0)
	}

	/// Will only work as root
	pub fn tryToReserveHugePages(&self, sys_path: &Path, hugePageSize: HugePageSize, count: u64) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Reserve '{}' huge pages of size '{:?}' reserved on NUMA node '{}'", count, hugePageSize, self.0));

		let file_path = self.numberOfHugePagesFilePath(sys_path, hugePageSize);
		file_path.write_value(count)
	}

	fn statisticListParse(&self, sys_path: &Path, statisticsFileName: &str) -> io::Result<NumaNodeStatistics>
	{
		let path = self.itemPath(sys_path, statisticsFileName);
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

	fn numberOfHugePagesFilePath(&self, sys_path: &Path, hugePageSize: HugePageSize) -> PathBuf
	{
		self.hugepagesPath(sys_path, hugePageSize, "nr_hugepages")
	}

	#[inline(always)]
	fn nodesSysPath(sys_path: &Path) -> PathBuf
	{
		let mut nodesSysPath = PathBuf::from(sys_path);
		nodesSysPath.push("devices/system/node");
		nodesSysPath
	}

	#[inline(always)]
	fn nodesItemSysPath(sys_path: &Path, item: &str) -> PathBuf
	{
		let mut nodesItemSysPath = Self::nodesSysPath(sys_path);
		nodesItemSysPath.push(item);
		nodesItemSysPath
	}

	#[inline(always)]
	fn nodeSysPath(&self, sys_path: &Path) -> PathBuf
	{
		Self::nodesItemSysPath(sys_path, &format!("node{}", self.0))
	}

	#[inline(always)]
	fn itemPath(&self, sys_path: &Path, item: &str) -> PathBuf
	{
		let mut path = self.nodeSysPath(sys_path);
		path.push(item);
		path
	}

	#[inline(always)]
	fn hugepagesPath(&self, sys_path: &Path, hugePageSize: HugePageSize, item: &str) -> PathBuf
	{
		self.itemPath(sys_path, &format!("hugepages/hugepages-{}kB/{}", hugePageSize.size(), item.to_owned()))
	}

	#[inline(always)]
	pub fn findInstalledNumberOfNumaSockets() -> usize
	{
		let mut currentMaximum = NumaSocketId::SocketZeroAlwaysExists;
		for logicalCoreIdentifier in 0..(MaximumLogicalCores as u32)
		{
			if let Some(numa_socket_id) = LogicalCore(logicalCoreIdentifier).optionalNumaSocketId()
			{
				if numa_socket_id > currentMaximum
				{
					currentMaximum = numa_socket_id;
				}
			}
		}

		(currentMaximum.0 + 1) as usize
	}

	#[inline(always)]
	pub fn choose<'a, V>(&'a self, from: &'a [V; NumaNode::MaximumNumaSockets]) -> &V
	{
		&from[self.0 as usize]
	}

	#[inline(always)]
	pub fn getSocketStatistics(&self) -> rte_malloc_socket_stats
	{
		let mut statistics = unsafe { uninitialized() };
		let result = unsafe { rte_malloc_get_socket_stats(self.as_c_int(), &mut statistics) };
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
	pub fn forCurrentLogicalCore() -> Option<NumaSocketId>
	{
		Self::fromU32(unsafe { rte_socket_id() })
	}

	#[inline(always)]
	pub fn from_i32(value: i32) -> Option<NumaSocketId>
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
		debug_assert!(value <= NumaNode::MaximumNumaSockets as u32, "value '{}' is equal to or exceeds NumaNode::MaximumNumaSockets, '{}'", value, NumaNode::MaximumNumaSockets);
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
