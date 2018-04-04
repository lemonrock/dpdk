// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonNumaMemory
{
}

impl NonNumaMemory
{
	pub fn supportedHugePageSizesLargestFirst(sysPath: &Path, defaultHugePageSize: Option<HugePageSize>) -> Vec<HugePageSize>
	{
		let length = HugePageSize::PotentiallySupportedHugePageSizesLargestFirst.len();
		
		let mut supported = Vec::with_capacity(length);
		
		for hugePageSize in HugePageSize::PotentiallySupportedHugePageSizesLargestFirst.iter()
		{
			if Self::numberOfNonNumaHugePages(sysPath, *hugePageSize).is_ok()
			{
				supported.push(*hugePageSize);
			}
		}
		
		if let Some(defaultHugePageSize) = defaultHugePageSize
		{
			let mut containsDefaultHugePageSize = false;
			for hugePageSize in supported.iter()
			{
				if *hugePageSize == defaultHugePageSize
				{
					containsDefaultHugePageSize = true;
					break;
				}
			}
		
			assert!(containsDefaultHugePageSize, "supported huge page sizes '{:?}' do not contain default '{:?}'", supported, defaultHugePageSize)
		}
		
		supported.shrink_to_fit();
		supported
	}
	
	/// Will only work as root
	pub fn tryToClearAllNonNumaHugePagesReserved(sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<()>
	{
		assertEffectiveUserIsRoot(&format!("Clear all non-NUMA huge pages of size '{:?}'", hugePageSize));
		Self::tryToReserveNonNumaHugePages(sysPath, hugePageSize, 0)
	}
	
	/// Will only work as root
	pub fn tryToReserveNonNumaHugePages(sysPath: &Path, hugePageSize: HugePageSize, count: u64) -> io::Result<()>
	{
		assertEffectiveUserIsRoot(&format!("Reserve '{}' non-NUMA huge pages of size '{:?}'", count, hugePageSize));

		let filePath = Self::nonNumaNumberOfHugePagesFilePath(sysPath, hugePageSize);
		writeValueToFile(&filePath, count)
	}
	
	pub fn numberOfNonNumaHugePages(sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = Self::nonNumaNumberOfHugePagesFilePath(sysPath, hugePageSize);
		readValueFromFile(&filePath)
	}
	
	pub fn numberOfNonNumaFreeHugePages(sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = Self::nonNumaHugePagesFilePath(sysPath, hugePageSize, "free_hugepages");
		readValueFromFile(&filePath)
	}
	
	pub fn numberOfNonNumaSurplusHugePages(sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = Self::nonNumaHugePagesFilePath(sysPath, hugePageSize, "surplus_hugepages");
		readValueFromFile(&filePath)
	}
	
	pub fn numberOfNonNumaReservedHugePages(sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = Self::nonNumaHugePagesFilePath(sysPath, hugePageSize, "resv_hugepages");
		readValueFromFile(&filePath)
	}
	
	pub fn numberOfNonNumaMemoryPolicyHugePages(sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = Self::nonNumaHugePagesFilePath(sysPath, hugePageSize, "nr_hugepages_mempolicy");
		readValueFromFile(&filePath)
	}
	
	pub fn numberOfNonNumaOvercommitHugePages(sysPath: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let filePath = Self::nonNumaHugePagesFilePath(sysPath, hugePageSize, "nr_overcommit_hugepages");
		readValueFromFile(&filePath)
	}
	
	fn nonNumaNumberOfHugePagesFilePath(sysPath: &Path, hugePageSize: HugePageSize) -> PathBuf
	{
		Self::nonNumaHugePagesFilePath(sysPath, hugePageSize, "nr_hugepages")
	}
	
	fn nonNumaHugePagesFilePath(sysPath: &Path, hugePageSize: HugePageSize, fileName: &str) -> PathBuf
	{
		let mut filePath = Self::nonNumaHugePagesFolderPath(sysPath, hugePageSize);
		filePath.push(fileName);
		filePath
	}
	
	fn nonNumaHugePagesFolderPath(sysPath: &Path, hugePageSize: HugePageSize) -> PathBuf
	{
		let mut filePath = PathBuf::from(sysPath);
		filePath.push(format!("kernel/mm/hugepages/hugepages-{}kB", hugePageSize.size()));
		filePath
	}
}
