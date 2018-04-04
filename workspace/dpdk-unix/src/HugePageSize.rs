// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// See https://en.wikipedia.org/wiki/Page_(computer_memory)#Huge_pages
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HugePageSize
{
	_1MB = 1024,
	_2MB = 2048,
	_4MB = 4096,
	_16MB = 16_384,
	_256MB = 262_144,
	_512MB = 524_288, // arm64 alternative
	_1GB = 1_048_576,
	_2GB = 2_097_152,
	_16GB = 16_777_216,
}

impl HugePageSize
{
	pub fn calculateNumberOfHugePages(&self, desiredNumberOfKiloBytes: u64) -> u64
	{
		let size = self.size();
		if size < desiredNumberOfKiloBytes
		{
			1
		}
		else
		{
			size / desiredNumberOfKiloBytes
		}
	}
}


impl HugePageSize
{
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
	
	pub fn fromProcMemInfoValue(value: u64) -> Option<Self>
	{
		match value
		{
			1024 => Some(HugePageSize::_1MB),
			2048 => Some(HugePageSize::_2MB),
			4096 => Some(HugePageSize::_4MB),
			16384 => Some(HugePageSize::_16MB),
			262144 => Some(HugePageSize::_256MB),
			524288 => Some(HugePageSize::_512MB),
			1048576 => Some(HugePageSize::_1GB),
			2097152 => Some(HugePageSize::_2GB),
			16777216 => Some(HugePageSize::_16GB),
			
			_ => None,
		}
	}
	
	pub fn to_str(&self) -> &'static str
	{
		match *self
		{
			HugePageSize::_1MB => "1MB",
			HugePageSize::_2MB => "2MB",
			HugePageSize::_4MB => "4MB",
			HugePageSize::_16MB => "16MB",
			HugePageSize::_256MB => "256MB",
			HugePageSize::_512MB => "512MB",
			HugePageSize::_1GB => "1GB",
			HugePageSize::_2GB => "2GB",
			HugePageSize::_16GB => "16GB",
		}
	}
	
	pub fn size(&self) -> u64
	{
		*self as u64
	}
}
