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
		Some(unsafe { transmute(ranks as u8) })
	}
	
	/// As an initialization argument.
	#[inline(always)]
	pub fn as_initialization_argument(self) -> ConstCStr
	{
		use self::MemoryRanks::*;
		
		match self
		{
			One => ConstCStr(b"1\0"),
			Two => ConstCStr(b"2\0"),
			Three => ConstCStr(b"3\0"),
			Four => ConstCStr(b"4\0"),
			Five => ConstCStr(b"5\0"),
			Six => ConstCStr(b"6\0"),
			Seven => ConstCStr(b"7\0"),
			Eight => ConstCStr(b"8\0"),
			Nine => ConstCStr(b"9\0"),
			Ten => ConstCStr(b"10\0"),
			Eleven => ConstCStr(b"11\0"),
			Twelve => ConstCStr(b"12\0"),
			Thirteen => ConstCStr(b"13\0"),
			Fourteen => ConstCStr(b"14\0"),
			Fifteen => ConstCStr(b"15\0"),
			Sixteen => ConstCStr(b"16\0"),
		}
	}
}
