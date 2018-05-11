// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Commonly supported huge page sizes for modern popular CPU architectures (x86, ARM, PowerPC).
///
/// See also <https://en.wikipedia.org/wiki/Page_(computer_memory)#Huge_pages>.
///
/// `repr(u64)` values are in KiloBytes.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HugePageSize
{
	/// 1MB.
	_1MB = 1024,
	
	/// 2MB.
	_2MB = 2048,
	
	/// 4MB.
	_4MB = 4096,
	
	/// 16MB.
	_16MB = 16_384,
	
	/// 256MB.
	_256MB = 262_144,
	
	/// 512MB.
	///
	/// aarch64 alternative.
	_512MB = 524_288,

	/// 1GB.
	_1GB = 1_048_576,
	
	/// 2GB.
	_2GB = 2_097_152,
	
	/// 16GB.
	_16GB = 16_777_216,
}

impl HugePageSize
{
	/// Potentially supported huge page sizes.
	pub const PotentiallySupportedHugePageSizesLargestFirst: [HugePageSize; 9] =
	[
		HugePageSize::_16GB,
		HugePageSize::_2GB,
		HugePageSize::_1GB,
		HugePageSize::_512MB,
		HugePageSize::_256MB,
		HugePageSize::_16MB,
		HugePageSize::_4MB,
		HugePageSize::_2MB,
		HugePageSize::_1MB,
	];
	
	/// Size in mega bytes.
	#[inline(always)]
	pub fn size_in_mega_bytes(self) -> u64
	{
		self.size_in_kilo_bytes() / 1024
	}
	
	/// Size in kilo bytes.
	#[inline(always)]
	pub fn size_in_kilo_bytes(self) -> u64
	{
		self as u64
	}
	
	/// Size in bytes.
	#[inline(always)]
	pub fn size_in_bytes(self) -> u64
	{
		self.size_in_kilo_bytes() * 1024
	}
	
	/// Calculate number of huge pages.
	#[inline(always)]
	pub fn calculate_number_of_huge_pages(&self, desired_number_of_kilo_bytes: u64) -> u64
	{
		let size_in_kilo_bytes = self.size_in_kilo_bytes();
		if size_in_kilo_bytes < desired_number_of_kilo_bytes
		{
			1
		}
		else
		{
			size_in_kilo_bytes / desired_number_of_kilo_bytes
		}
	}
	
	/// Converts a value from Linux's `/proc/mem` pseudo-file into a `HugePageSize`.
	#[inline(always)]
	pub fn from_proc_mem_info_value(value: u64) -> Option<Self>
	{
		use self::HugePageSize::*;
		
		match value
		{
			1024 => Some(_1MB),
			2048 => Some(_2MB),
			4096 => Some(_4MB),
			16384 => Some(_16MB),
			262144 => Some(_256MB),
			524288 => Some(_512MB),
			1048576 => Some(_1GB),
			2097152 => Some(_2GB),
			16777216 => Some(_16GB),
			
			_ => None,
		}
	}
	
	/// String description including unit.
	#[inline(always)]
	pub fn to_str(&self) -> &'static str
	{
		use self::HugePageSize::*;
		
		match *self
		{
			_1MB => "1MB",
			_2MB => "2MB",
			_4MB => "4MB",
			_16MB => "16MB",
			_256MB => "256MB",
			_512MB => "512MB",
			_1GB => "1GB",
			_2GB => "2GB",
			_16GB => "16GB",
		}
	}
	
	/// Supported huge page sizes, sorted smallest to largest.
	#[inline(always)]
	pub fn largest_supported_huge_page_size(sys_path: &SysPath) -> Self
	{
		*Self::supported_huge_page_sizes(sys_path).iter().rev().next().expect("Huge pages are not supported")
	}
	
	/// Supported huge page sizes, sorted smallest to largest.
	#[inline(always)]
	pub fn supported_huge_page_sizes(sys_path: &SysPath) -> BTreeSet<Self>
	{
		let mut supported = BTreeSet::new();
		
		for huge_page_size in Self::PotentiallySupportedHugePageSizesLargestFirst.iter()
		{
			if let Ok(_) = huge_page_size.number_of_global_huge_pages(sys_path)
			{
				supported.insert(*huge_page_size);
			}
		}
		
		supported
	}

	/// Try to unreserve (clear reservations of) global huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn unreserve_global_huge_pages(self, sys_path: &SysPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Clear all global huge pages of size '{:?}'", self));
		
		self.reserve_global_huge_pages(sys_path, 0)
	}
	
	/// Try to reserve global huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn reserve_global_huge_pages(self, sys_path: &SysPath, number_to_try_to_reserve: u64) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Reserve '{}' global huge pages of size '{:?}'", number_to_try_to_reserve, self));
		
		sys_path.global_hugepages_file_path(self, "nr_hugepages").write_value(number_to_try_to_reserve);
		
		#[cfg(target_arch = "powerpc64")]
		{
			sys_path.global_hugepages_file_path(self, "nr_overcommit_hugepages").write_value(number_to_try_to_reserve)
		}
	}
	
	/// Read number of global huge pages of `self` size.
	#[inline(always)]
	pub fn number_of_global_huge_pages(self, sys_path: &SysPath) -> io::Result<u64>
	{
		sys_path.read_global_hugepages_value(self, "nr_hugepages")
	}
	
	/// Read number of free global huge pages of `self` size.
	#[inline(always)]
	pub fn number_of_free_global_huge_pages(self, sys_path: &SysPath) -> io::Result<u64>
	{
		sys_path.read_global_hugepages_value(self, "free_hugepages")
	}
	
	/// Read number of surplus global huge pages of `self` size.
	#[inline(always)]
	pub fn number_of_surplus_global_huge_pages(self, sys_path: &SysPath) -> io::Result<u64>
	{
		sys_path.read_global_hugepages_value(self, "surplus_hugepages")
	}
	
	/// Read number of reserved global huge pages of `self` size.
	#[inline(always)]
	pub fn number_of_reserved_global_huge_pages(self, sys_path: &SysPath) -> io::Result<u64>
	{
		sys_path.read_global_hugepages_value(self, "resv_hugepages")
	}
	
	/// Read number of memory policy global huge pages of `self` size.
	#[inline(always)]
	pub fn number_of_memory_policy_global_huge_pages(self, sys_path: &SysPath) -> io::Result<u64>
	{
		sys_path.read_global_hugepages_value(self, "nr_hugepages_mempolicy")
	}
	
	/// Read number of overcommit global huge pages of `self` size.
	#[inline(always)]
	pub fn number_of_overcommit_global_huge_pages(self, sys_path: &SysPath) -> io::Result<u64>
	{
		sys_path.read_global_hugepages_value(self, "nr_overcommit_hugepages")
	}
	
	/// Try to unreserve (clear reservations of) NUMA huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn unreserve_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Clear all NUMA huge pages of size '{:?}'", self));
		
		self.reserve_numa_huge_pages(sys_path, numa_node, 0)
	}
	
	/// Try to reserve NUMA huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn reserve_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8, number_to_try_to_reserve: u64) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Reserve '{}' NUMA huge pages of size '{:?}'", number_to_try_to_reserve, self));
		
		sys_path.numa_hugepages_file_path(self, numa_node, "nr_hugepages").write_value(number_to_try_to_reserve)
	}
	
	/// Read number of NUMA huge pages of `self` size.
	///
	/// This will fail if this is not a NUMA-based machine or the node is not present.
	#[inline(always)]
	pub fn number_of_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8) -> io::Result<u64>
	{
		sys_path.read_numa_hugepages_value(self, numa_node, "nr_hugepages")
	}
	
	/// Read number of free NUMA node huge pages of `self` size.
	///
	/// This will fail if this is not a NUMA-based machine or the node is not present.
	#[inline(always)]
	pub fn number_of_free_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8) -> io::Result<u64>
	{
		sys_path.read_numa_hugepages_value(self, numa_node, "free_hugepages")
	}
	
	/// Read number of surplus NUMA huge pages of `self` size.
	///
	/// This will fail if this is not a NUMA-based machine or the node is not present.
	#[inline(always)]
	pub fn number_of_surplus_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8) -> io::Result<u64>
	{
		sys_path.read_numa_hugepages_value(self, numa_node, "surplus_hugepages")
	}
}
