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

impl TryFrom<u8> for InternetProtocolVersion6MulticastAddressScope
{
	type Error = TryFromIntError;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		use self::InternetProtocolVersion6MulticastAddressScope::*;
		
		let this = match value
		{
			0x1 => InterfaceLocal,
			
			0x2 => LinkLocal,
			
			0x3 => RealmLocal,
			
			0x4 => AdminLocal,
			
			0x5 => SiteLocal,
			
			0x8 => OrganisationLocal,
			
			0xE => Global,
			
			_ => return Err(TryFromIntError),
		};
		Ok(this)
	}
}

impl Display for InternetProtocolVersion6MulticastAddressScope
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::InternetProtocolVersion6MulticastAddressScope::*;
		
		match *self
		{
			InterfaceLocal => "interface local",
			
			LinkLocal => "link local",
			
			RealmLocal => "realm local",
			
			SiteLocal => "site local",
			
			OrganisationLocal => "organisation local",
			
			Global => "global",
		}
		
		write!(f, "{}", string)
	}
}

impl Default for InternetProtocolVersion6MulticastAddressScope
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6MulticastAddressScope::InterfaceLocal
	}
}

impl InternetProtocolVersion6MulticastAddressScope
{
	/// Is this an interface-local, ie loopback, multicast address?
	#[inline(always)]
	pub fn is_interface_local_also_known_as_loopback(self)
	{
		self == InternetProtocolVersion6MulticastAddressScope::InterfaceLocal
	}
}
