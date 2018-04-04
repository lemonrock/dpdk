// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ideally, need to boot the kernel with   default_hugepagesz=1GB hugepagesz=1GB hugepages=X  (forces default to 1GB rather than 2MB) for x86_64 (not for virtual machines)
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct HugePagesConfiguration
{
	pub enableTransparentHugePages: bool,
	pub nonNumaHugePageAllocationStrategy: HugePageAllocationStrategy,
	pub numaHugePageAllocationStrategy: HugePageAllocationStrategy,
	pub hugePageMountSettings: HugePageMountSettings,
}

impl Default for HugePagesConfiguration
{
	fn default() -> Self
	{
		HugePagesConfiguration
		{
			enableTransparentHugePages: false,
			nonNumaHugePageAllocationStrategy: HugePageAllocationStrategy::TenPercentRatioOfTotalFree,
			numaHugePageAllocationStrategy: HugePageAllocationStrategy::EightyPercentRatioOfTotalFree,
			hugePageMountSettings: HugePageMountSettings::default(),
		}
	}
}

impl HugePagesConfiguration
{
	#[cfg(any(target_os = "android", target_os = "linux"))]
	fn setUpHugePagesAndNumaMemory(&self, procPath: &Path, sysPath: &Path, numaSockets: &NumaSockets) -> (Option<PathBuf>, HugePageFinisher)
	{
		let nonNumaHugePageAllocationStrategy = &self.nonNumaHugePageAllocationStrategy;
		let numaHugePageAllocationStrategy = &self.numaHugePageAllocationStrategy;
		
		fn verifyLinuxKernelSupportsHugetlbfs(procPath: &Path)
		{
			match FileSystemType::parse(procPath).expect("Could not parse list of filesystems").get(&FileSystemType::hugetlbfs)
			{
				None => panic!("Linux kernel does not support file system 'hugetlbfs'"),
				Some(hasNoAssociatedDevice) => if !hasNoAssociatedDevice
				{
					panic!("File system 'hugetlbfs' has associated devices (ie is not 'nodev')");
				},
			}
		}
		
		fn mountHugePages<'a>(mounts: &'a HashMap<PathBuf, Mount>, hugePageMountSettings: &HugePageMountSettings, largestHugePageSize: HugePageSize) -> (PathBuf, HugePageFinisher)
		{
			fn findAHugeTlbFsMount<'a>(mounts: &'a HashMap<PathBuf, Mount>) -> Option<&'a Path>
			{
				for (_, mount) in mounts.iter()
				{
					if mount.isFileSystemType(&FileSystemType::hugetlbfs)
					{
						let mountPoint = &mount.mountPoint;
						if mountPoint.is_dir()
						{
							return Some(mountPoint);
						}
					}
				}
				None
			}
			
			match findAHugeTlbFsMount(mounts)
			{
				Some(mountPath) =>
				{
					(PathBuf::from(mountPath), HugePageFinisher::FreeBsd)
				},
				None =>
				{
					let mountPoint = &hugePageMountSettings.mountPoint;
					let created =
					{
						if mountPoint.exists()
						{
							if !mountPoint.is_dir()
							{
								panic!("Mount point {:?} for Hugeltbfs is not a directory", mountPoint);
							}
							false
						}
						else
						{
							create_dir_all(mountPoint).expect(&format!("Could not create Hugetlbfs mountPoint at {:?}", mountPoint));
							true
						}
					};
					
					(Mount::mountHugePages(hugePageMountSettings, Some(largestHugePageSize)).expect("Could not mount hugetlbfs"), HugePageFinisher::new(mountPoint, created, true))
				}
			}
		}
		
		let machineMemoryStatistics = MemoryStatistics::forMachine(procPath).expect("Could not parse memory statistics");
		
		let supportedHugePageSizes = NonNumaMemory::supportedHugePageSizesLargestFirst(sysPath, machineMemoryStatistics.defaultHugePageSize());
		if supportedHugePageSizes.is_empty()
		{
			return (None, HugePageFinisher::FreeBsd);
		}
		
		let largestHugePageSize = supportedHugePageSizes[0];
		
		verifyLinuxKernelSupportsHugetlbfs(procPath);
		
		numaSockets.iterateUsefulSocketsIfIsANumaMachine(|numaSocketId|
		{
			numaSocketId.tryToCompact(sysPath).expect("Could not write to compact (1)");
			numaSocketId.tryToEvictPages(sysPath).expect("Could not write to evict (1)");
			numaSocketId.tryToCompact(sysPath).expect("Could not write to compact (2)");
			numaSocketId.tryToEvictPages(sysPath).expect("Could not write to evict (2)");
			for hugePageSize in supportedHugePageSizes.iter()
			{
				numaSocketId.tryToClearAllHugePagesReserved(&sysPath, *hugePageSize).expect(&format!("Could not clear NUMA huge pages of size '{:?}' on socket '{}'", hugePageSize, numaSocketId.as_c_uint()));
			}
		});
		
		for hugePageSize in supportedHugePageSizes.iter()
		{
			NonNumaMemory::tryToClearAllNonNumaHugePagesReserved(sysPath, *hugePageSize).expect(&format!("Could not clear Non-NUMA huge pages of size '{:?}'", hugePageSize));
		}
		
		let machineTotalFreeMemory = machineMemoryStatistics.freePhysicalRam().expect("No machine total free RAM statistic");
		let count = nonNumaHugePageAllocationStrategy.allocateInPages(largestHugePageSize, machineTotalFreeMemory);
		NonNumaMemory::tryToReserveNonNumaHugePages(sysPath, largestHugePageSize, count).expect("Could not reserve non-NUMA huge pages");
		
		numaSockets.iterateUsefulSocketsIfIsANumaMachine(|numaSocketId|
		{
			let numaNodeTotalFreeMemory = numaSocketId.meminfo(sysPath).expect(&format!("Could not parse NUMA node memory statistics on socket '{}'", numaSocketId.as_c_uint())).freePhysicalRam().expect(&format!("No NUMA node total free RAM statistic on socket '{}'", numaSocketId.as_c_uint()));
			let count = numaHugePageAllocationStrategy.allocateInPages(largestHugePageSize, numaNodeTotalFreeMemory);
			numaSocketId.tryToReserveHugePages(sysPath, largestHugePageSize, count).expect(&format!("Could not reserve NUMA huge pages on socket '{}'", numaSocketId.as_c_uint()));
		});
		
		adjustTransparentHugePages(self.enableTransparentHugePages);
		
		let mounts = Mounts::parse(procPath).expect("Could not parse mounts");
		let (mountPath, finisher) = mountHugePages(&mounts, &self.hugePageMountSettings, largestHugePageSize);
		(Some(mountPath), finisher)
	}
}
