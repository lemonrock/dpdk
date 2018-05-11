// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Restricted enumeration of number of memory ranks.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum MemoryRanks
{
	#[allow(missing_docs)]
	One = 1,
	
	#[allow(missing_docs)]
	Two = 2,
	
	#[allow(missing_docs)]
	Three = 3,
	
	#[allow(missing_docs)]
	Four = 4,
	
	#[allow(missing_docs)]
	Five = 5,
	
	#[allow(missing_docs)]
	Six = 6,
	
	#[allow(missing_docs)]
	Seven = 7,
	
	#[allow(missing_docs)]
	Eight = 8,
	
	#[allow(missing_docs)]
	Nine = 9,
	
	#[allow(missing_docs)]
	Ten = 10,
	
	#[allow(missing_docs)]
	Eleven = 11,
	
	#[allow(missing_docs)]
	Twelve = 12,
	
	#[allow(missing_docs)]
	Thirteen = 13,
	
	#[allow(missing_docs)]
	Fourteen = 14,
	
	#[allow(missing_docs)]
	Fifteen = 15,
	
	#[allow(missing_docs)]
	Sixteen = 16,
}

impl MemoryRanks
{
	/// Configured number of memory ranks.
	///
	/// Returns None if differs across memory segments or devices.
	#[inline(always)]
	pub fn configured_number_of_memory_ranks() -> Option<MemoryRanks>
	{
		let ranks = unsafe { rte_memory_get_nrank() };
		if ranks == 0
		{
			return None
		}
		if ranks > 16
		{
			panic!("Invalid number of memory ranks '{}'", ranks)
		}
		Some(unsafe { transmute(ranks) })
	}
	
	#[inline(always)]
	pub(crate) fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::MemoryRanks::*;
		
		const_cstr!
		{
			_1 = "1";
			_2 = "2";
			_3 = "3";
			_4 = "4";
			_5 = "5";
			_6 = "6";
			_7 = "7";
			_8 = "8";
			_9 = "9";
			_10 = "10";
			_11 = "11";
			_12 = "12";
			_13 = "13";
			_14 = "14";
			_15 = "15";
			_16 = "16";
		}
		
		match self
		{
			One => _1,
			Two => _2,
			Three => _3,
			Four => _4,
			Five => _5,
			Six => _6,
			Seven => _7,
			Eight => _8,
			Nine => _9,
			Ten => _10,
			Eleven => _11,
			Twelve => _12,
			Thirteen => _13,
			Fourteen => _14,
			Fifteen => _15,
			Sixteen => _16,
		}
	}
}
