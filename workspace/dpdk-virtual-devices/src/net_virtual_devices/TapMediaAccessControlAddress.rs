// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux TAP media access control address.
///
/// Defaults to `Random`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum TapMediaAccessControlAddress
{
	/// Random address is assigned.
	Random,
	
	/// Fixed address, auto-incrementing from `00:64:74:61:70:00` for each interface created.
	Fixed,
	
	/// Last byte of address specified in `00:64:74:61:70:XX` where `XX` is the last byte.
	LastByteSpecified(u8),
}

impl Default for TapMediaAccessControlAddress
{
	#[inline(always)]
	fn default() -> Self
	{
		TapMediaAccessControlAddress::Random
	}
}

impl TapMediaAccessControlAddress
{
	#[inline(always)]
	pub(crate) fn to_string(&self) -> String
	{
		use self::TapMediaAccessControlAddress::*;
		
		match *self
		{
			Random => "".to_string(),
			
			Fixed => ",mac=fixed".to_string(),
			
			LastByteSpecified(byte) => format!(",mac=\"00:64:74:61:70:{:02X}\"", byte),
		}
	}
}
