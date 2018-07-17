// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A virtual Lan identifier 1 - 4094 inclusive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtualLanIdentifier(u16);

impl VirtualLanIdentifier
{
	/// Parse.
	///
	/// Returns an error if zero or > 4094.
	#[inline(always)]
	pub fn new(value: u16) -> Result<VirtualLanIdentifier, ()>
	{
		if value >= 1 && value <= 4094
		{
			Ok(VirtualLanIdentifier(value))
		}
		else
		{
			Err(())
		}
	}
	
	/// To u16.
	#[inline(always)]
	pub fn to_u16(self) -> u16
	{
		self.0
	}
}
