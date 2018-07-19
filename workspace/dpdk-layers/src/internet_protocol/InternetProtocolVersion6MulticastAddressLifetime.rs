// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Protocol (IP) version 6 multicast address lifetime.
///
/// Defaults to Temporary.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum InternetProtocolVersion6MulticastAddressLifetime
{
	#[allow(missing_docs)]
	Permanent = 0x0,
	
	#[allow(missing_docs)]
	Temporary = 0x1,
}

impl Display for InternetProtocolVersion6MulticastAddressLifetime
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::InternetProtocolVersion6MulticastAddressLifetime::*;
		
		match *self
		{
			Permanent => "permanent",
			
			Temporary => "temporary",
		}
		
		write!(f, "{}", string)
	}
}

impl TryFrom<u8> for InternetProtocolVersion6MulticastAddressLifetime
{
	type Error = TryFromIntError;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value > 1
		{
			Err(TryFromIntError)
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl From<bool> for InternetProtocolVersion6MulticastAddressLifetime
{
	#[inline(always)]
	fn from(value: bool) -> Self
	{
		use self::InternetProtocolVersion6MulticastAddressLifetime::*;
		
		if value
		{
			Permanent
		}
		else
		{
			Temporary
		}
	}
}

impl Into<u8> for InternetProtocolVersion6MulticastAddressLifetime
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Into<bool> for InternetProtocolVersion6MulticastAddressLifetime
{
	#[inline(always)]
	fn into(self) -> bool
	{
		use self::InternetProtocolVersion6MulticastAddressLifetime::*;
		
		match self
		{
			Permanent => true,
			
			Temporary => false,
		}
	}
}

impl Default for InternetProtocolVersion6MulticastAddressLifetime
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6MulticastAddressLifetime::Temporary
	}
}
