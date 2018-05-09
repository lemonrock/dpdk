// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a finite quantity or infinite (ie no) limit for a resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ResourceLimit
{
	/// A finite limit; may be zero.
	Finite(u64),
	
	/// An infinite limit, ie no limit.
	///
	/// Not all resources support an infinite limit.
	///
	/// Also used to signify 'true' for resources which have an on-off setting.
	Infinite,
}

impl ResourceLimit
{
	const Infinity: rlim64_t = ::libc::RLIM_INFINITY as rlim64_t;
	
	/// Obtains the maximum number of file descriptors as a finite resource limit.
	pub fn maximum_number_of_open_file_descriptors(proc_path: &ProcPath) -> Result<ResourceLimit, io::Error>
	{
		Ok(ResourceLimit::Finite(proc_path.maximum_number_of_open_file_descriptors()?))
	}
	
	/// Value.
	#[inline(always)]
	pub fn value(&self) -> u64
	{
		use self::ResourceLimit::*;
		
		match *self
		{
			Finite(limit) => limit,
			Infinite => ::std::u64::MAX,
		}
	}
	
	#[inline(always)]
	pub(crate) fn convert(value: rlim64_t) -> ResourceLimit
	{
		use self::ResourceLimit::*;
		
		if value >= Self::Infinity
		{
			Infinite
		}
		else
		{
			Finite(value)
		}
	}
	
	#[inline(always)]
	pub(crate) fn unwrap(&self) -> rlim64_t
	{
		use self::ResourceLimit::*;
		
		match *self
		{
			Finite(limit) =>
			{
				assert!(limit < Self::Infinity, "limit '{}' equals or exceeds Infinity '{}'", limit, Self::Infinity);
				limit
			},
			Infinite => Self::Infinity
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_finite(&self) -> bool
	{
		match *self
		{
			ResourceLimit::Finite(_) => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_infinite(&self) -> bool
	{
		match *self
		{
			ResourceLimit::Infinite => true,
			_ => false,
		}
	}
}
