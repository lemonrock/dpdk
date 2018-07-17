// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Protocol (IP) version 6 multicast address scope.
///
/// Defaults to Node.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum InternetProtocolVersion6MulticastAddressScope
{
	/// Also known as node-local.
	InterfaceLocal = 0x1,
	
	#[allow(missing_docs)]
	LinkLocal = 0x2,
	
	/// Added by RFC 7346.
	RealmLocal = 0x3,
	
	/// Added by RFC 7346.
	AdminLocal = 0x4,
	
	#[allow(missing_docs)]
	SiteLocal = 0x5,
	
	#[allow(missing_docs)]
	OrganisationLocal = 0x8,
	
	/// ie the Internet.
	Global = 0xE,
}

impl Default for InternetProtocolVersion6MulticastAddressScope
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6MulticastAddressScope::InterfaceLocal
	}
}
