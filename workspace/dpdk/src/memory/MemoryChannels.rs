// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Restricted enumeration of number of memory channels.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum MemoryChannels
{
	#[allow(missing_docs)]
	One = 1,
	
	#[allow(missing_docs)]
	Two = 2,
	
	#[allow(missing_docs)]
	Three = 3,
	
	#[allow(missing_docs)]
	Four = 4,
}

impl MemoryChannels
{
	/// Configured number of memory channels.
	///
	/// Returns None if differs across memory segments or devices.
	#[inline(always)]
	pub fn configured_number_of_memory_channels() -> Option<MemoryChannels>
	{
		let channels = unsafe { rte_memory_get_nchannel() };
		if channels == 0
		{
			return None
		}
		if channels > 4
		{
			panic!("Invalid number of memory channels '{}'", channels)
		}
		Some(unsafe { transmute(channels) })
	}
	
	#[inline(always)]
	pub(crate) fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::MemoryChannels::*;
		
		const_cstr!
		{
			_1 = "1";
			_2 = "2";
			_3 = "3";
			_4 = "4";
		}
		
		match self
		{
			One => _1,
			Two => _2,
			Three => _3,
			Four => _4,
		}
	}
}
