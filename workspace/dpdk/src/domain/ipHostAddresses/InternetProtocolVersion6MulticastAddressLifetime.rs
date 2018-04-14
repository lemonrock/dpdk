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

impl Default for InternetProtocolVersion6MulticastAddressLifetime
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6MulticastAddressLifetime::Temporary
	}
}
