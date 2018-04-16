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
