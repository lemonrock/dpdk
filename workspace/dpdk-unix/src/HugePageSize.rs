// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Commonly supported huge page sizes for modern popular CPU architectures (x86, ARM, PowerPC).
///
/// See also <https://en.wikipedia.org/wiki/Page_(computer_memory)#Huge_pages>.
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
	
	/// Calculate number of huge pages.
	#[inline(always)]
	pub fn calculate_number_of_huge_pages(&self, desired_number_of_kilo_bytes: u64) -> u64
	{
		let size = self.size();
		if size < desired_number_of_kilo_bytes
		{
			1
		}
		else
		{
			size / desired_number_of_kilo_bytes
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
	
	/// Size in bytes.
	#[inline(always)]
	pub fn size(&self) -> u64
	{
		*self as u64
	}
}
