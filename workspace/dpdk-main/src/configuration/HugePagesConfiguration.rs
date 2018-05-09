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
	fn setUpHugePagesAndNumaMemory(&self, proc_path: &ProcPath, sys_path: &SysPath, numa_sockets: &NumaSockets) -> (Option<PathBuf>, HugePageFinisher)
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

		fn mount_huge_pages<'a>(mounts: &'a HashMap<PathBuf, Mount>, hugePageMountSettings: &HugePageMountSettings, largestHugePageSize: HugePageSize) -> (PathBuf, HugePageFinisher)
		{
			fn findAHugeTlbFsMount<'a>(mounts: &'a HashMap<PathBuf, Mount>) -> Option<&'a Path>
			{
				for (_, mount) in mounts.iter()
				{
					if mount.has_file_system_type(&FileSystemType::hugetlbfs)
					{
						let mountPoint = &mount.mount_point;
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
					let mount_point = &hugePageMountSettings.mount_point;
					let created =
					{
						if mount_point.exists()
						{
							if !mount_point.is_dir()
							{
								panic!("Mount point {:?} for hugeltbfs is not a directory", mount_point);
							}
							false
						}
						else
						{
							create_dir_all(mount_point).expect(&format!("Could not create hugeltbfs mount_point at {:?}", mount_point));
							true
						}
					};

					(Mount::mount_huge_pages(hugePageMountSettings, Some(largestHugePageSize)).expect("Could not mount hugetlbfs"), HugePageFinisher::new(mount_point, created, true))
				}
			}
		}

		let machineMemoryStatistics = MemoryStatistics::parse_for_machine(proc_path).expect("Could not parse memory statistics");

		let supportedHugePageSizes = NonNumaMemory::supportedHugePageSizesLargestFirst(sys_path, machineMemoryStatistics.default_huge_page_size());
		if supportedHugePageSizes.is_empty()
		{
			return (None, HugePageFinisher::FreeBsd);
		}

		let largestHugePageSize = supportedHugePageSizes[0];

		verifyLinuxKernelSupportsHugetlbfs(proc_path);

		numa_sockets.iterateUsefulSocketsIfIsANumaMachine(|numa_socket_id|
		{
			numa_socket_id.tryToCompact(sys_path).expect("Could not write to compact (1)");
			numa_socket_id.tryToEvictPages(sys_path).expect("Could not write to evict (1)");
			numa_socket_id.tryToCompact(sys_path).expect("Could not write to compact (2)");
			numa_socket_id.tryToEvictPages(sys_path).expect("Could not write to evict (2)");
			for hugePageSize in supportedHugePageSizes.iter()
			{
				numa_socket_id.tryToClearAllHugePagesReserved(&sys_path, *hugePageSize).expect(&format!("Could not clear NUMA huge pages of size '{:?}' on socket '{}'", hugePageSize, numa_socket_id.as_c_uint()));
			}
		});

		for hugePageSize in supportedHugePageSizes.iter()
		{
			NonNumaMemory::tryToClearAllNonNumaHugePagesReserved(sys_path, *hugePageSize).expect(&format!("Could not clear Non-NUMA huge pages of size '{:?}'", hugePageSize));
		}

		let machineTotalFreeMemory = machineMemoryStatistics.free_physical_ram().expect("No machine total free RAM statistic");
		let count = nonNumaHugePageAllocationStrategy.allocateInPages(largestHugePageSize, machineTotalFreeMemory);
		NonNumaMemory::tryToReserveNonNumaHugePages(sys_path, largestHugePageSize, count).expect("Could not reserve non-NUMA huge pages");

		numa_sockets.iterateUsefulSocketsIfIsANumaMachine(|numa_socket_id|
		{
			let numaNodeTotalFreeMemory = numa_socket_id.meminfo(sys_path).expect(&format!("Could not parse NUMA node memory statistics on socket '{}'", numa_socket_id.as_c_uint())).free_physical_ram().expect(&format!("No NUMA node total free RAM statistic on socket '{}'", numa_socket_id.as_c_uint()));
			let count = numaHugePageAllocationStrategy.allocateInPages(largestHugePageSize, numaNodeTotalFreeMemory);
			numa_socket_id.tryToReserveHugePages(sys_path, largestHugePageSize, count).expect(&format!("Could not reserve NUMA huge pages on socket '{}'", numa_socket_id.as_c_uint()));
		});

		adjust_transparent_huge_pages(self.enableTransparentHugePages);

		let mounts = Mounts::parse(proc_path).expect("Could not parse mounts");
		let (mountPath, finisher) = mount_huge_pages(&mounts, &self.hugePageMountSettings, largestHugePageSize);
		(Some(mountPath), finisher)
	}
}
