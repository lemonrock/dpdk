// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonNumaMemory
{
}

impl NonNumaMemory
{
	pub fn supportedHugePageSizesLargestFirst(sys_path: &Path, default_huge_page_size: Option<HugePageSize>) -> Vec<HugePageSize>
	{
		let length = HugePageSize::PotentiallySupportedHugePageSizesLargestFirst.len();
		
		let mut supported = Vec::with_capacity(length);
		
		for hugePageSize in HugePageSize::PotentiallySupportedHugePageSizesLargestFirst.iter()
		{
			if Self::numberOfNonNumaHugePages(sys_path, *hugePageSize).is_ok()
			{
				supported.push(*hugePageSize);
			}
		}
		
		if let Some(default_huge_page_size) = default_huge_page_size
		{
			let mut containsDefaultHugePageSize = false;
			for hugePageSize in supported.iter()
			{
				if *hugePageSize == default_huge_page_size
				{
					containsDefaultHugePageSize = true;
					break;
				}
			}
		
			assert!(containsDefaultHugePageSize, "supported huge page sizes '{:?}' do not contain default '{:?}'", supported, default_huge_page_size)
		}
		
		supported.shrink_to_fit();
		supported
	}
	
	/// Will only work as root
	pub fn tryToClearAllNonNumaHugePagesReserved(sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Clear all non-NUMA huge pages of size '{:?}'", hugePageSize));
		Self::tryToReserveNonNumaHugePages(sys_path, hugePageSize, 0)
	}
	
	/// Will only work as root
	pub fn tryToReserveNonNumaHugePages(sys_path: &Path, hugePageSize: HugePageSize, count: u64) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Reserve '{}' non-NUMA huge pages of size '{:?}'", count, hugePageSize));

		let file_path = Self::nonNumaNumberOfHugePagesFilePath(sys_path, hugePageSize);
		file_path.write_value(count)
	}
	
	pub fn numberOfNonNumaHugePages(sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = Self::nonNumaNumberOfHugePagesFilePath(sys_path, hugePageSize);
		file_path.read_value()
	}
	
	pub fn numberOfNonNumaFreeHugePages(sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = Self::nonNumaHugePagesFilePath(sys_path, hugePageSize, "free_hugepages");
		file_path.read_value()
	}
	
	pub fn numberOfNonNumaSurplusHugePages(sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = Self::nonNumaHugePagesFilePath(sys_path, hugePageSize, "surplus_hugepages");
		file_path.read_value()
	}
	
	pub fn numberOfNonNumaReservedHugePages(sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = Self::nonNumaHugePagesFilePath(sys_path, hugePageSize, "resv_hugepages");
		file_path.read_value()
	}
	
	pub fn numberOfNonNumaMemoryPolicyHugePages(sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = Self::nonNumaHugePagesFilePath(sys_path, hugePageSize, "nr_hugepages_mempolicy");
		file_path.read_value()
	}
	
	pub fn numberOfNonNumaOvercommitHugePages(sys_path: &Path, hugePageSize: HugePageSize) -> io::Result<u64>
	{
		let file_path = Self::nonNumaHugePagesFilePath(sys_path, hugePageSize, "nr_overcommit_hugepages");
		file_path.read_value()
	}
	
	fn nonNumaNumberOfHugePagesFilePath(sys_path: &Path, hugePageSize: HugePageSize) -> PathBuf
	{
		Self::nonNumaHugePagesFilePath(sys_path, hugePageSize, "nr_hugepages")
	}
	
	fn nonNumaHugePagesFilePath(sys_path: &Path, hugePageSize: HugePageSize, fileName: &str) -> PathBuf
	{
		let mut file_path = Self::nonNumaHugePagesFolderPath(sys_path, hugePageSize);
		file_path.push(fileName);
		file_path
	}
	
	fn nonNumaHugePagesFolderPath(sys_path: &Path, hugePageSize: HugePageSize) -> PathBuf
	{
		let mut file_path = PathBuf::from(sys_path);
		file_path.push(format!("kernel/mm/hugepages/hugepages-{}kB", hugePageSize.size()));
		file_path
	}
}
