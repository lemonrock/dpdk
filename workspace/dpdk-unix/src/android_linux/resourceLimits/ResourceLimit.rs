// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ResourceLimit
{
	Finite(u64),
	Infinite,
}

impl ResourceLimit
{
	const Infinity: rlim64_t = ::libc::RLIM_INFINITY as rlim64_t;

	pub fn maximumNumberOfFileDescriptors(procPath: &Path) -> Result<ResourceLimit, io::Error>
	{
		let mut nrOpenFilePath = PathBuf::from(procPath);
		nrOpenFilePath.push("sys/fs/nr_open");
		let value: u64 = try!(readValueFromFile(&nrOpenFilePath));
		Ok(ResourceLimit::Finite(value))
	}
	
	pub fn value(&self) -> u64
	{
		match *self
		{
			ResourceLimit::Finite(limit) => limit,
			ResourceLimit::Infinite => ::std::u64::MAX,
		}
	}
	
	pub fn convert(value: rlim64_t) -> ResourceLimit
	{
		if value >= Self::Infinity
		{
			ResourceLimit::Infinite
		}
		else
		{
			ResourceLimit::Finite(value)
		}
	}
	
	pub fn unwrap(&self) -> rlim64_t
	{
		match *self
		{
			ResourceLimit::Finite(limit) =>
			{
				assert!(limit < Self::Infinity, "limit '{}' equals or exceeds Infinity '{}'", limit, Self::Infinity);
				limit
			},
			ResourceLimit::Infinite => Self::Infinity
		}
	}
	
	pub fn isFinite(&self) -> bool
	{
		match *self
		{
			ResourceLimit::Finite(_) => true,
			_ => false,
		}
	}
	
	pub fn isInfinite(&self) -> bool
	{
		match *self
		{
			ResourceLimit::Infinite => true,
			_ => false,
		}
	}
}
