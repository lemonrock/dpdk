// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum Processor
{
	/// 80386.
	_386 = 0x00,
	
	/// 80486.
	_486 = 0x01,
	
	Pentium = 0x02,
	
	/// ?
	PentiumAlt = 0x03,
	
	/// ?
	P6 = 0x04,
	
	Alpha = 0x10,
	
	PowerPC = 0x20,
	
	/// MIPS.
	Mips = 0x30,
	
	/// Also exists as a class...
	Coprocessor = 0x40,
}
