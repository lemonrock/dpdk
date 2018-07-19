// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension header type or Layer 4 protocol number (the range overlaps; yuck).
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	/// A known extension header type.
	pub extension_header_type: ExtensionHeaderType,
	
	/// A known layer 4 protocol number.
	pub layer_4_protocol_number: Layer4ProtocolNumber,
	
	/// An unknown extension header type or layer 4 protocol number.
	pub unknown: u8,
}

impl From<u8> for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		Self
		{
			unknown: value,
		}
	}
}

impl Into<u8> for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn into(self) -> u8
	{
		unsafe { self.unknown }
	}
}

impl Display for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.unknown })
	}
}

impl Debug for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.unknown })
	}
}

impl PartialOrd for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.unknown.partial_cmp(&other.unknown) }
	}
}

impl Ord for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.unknown.cmp(&other.unknown) }
	}
}

impl PartialEq for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.unknown == other.unknown }
	}
}

impl Eq for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
}

impl Hash for ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u8(unsafe { self.unknown })
	}
}
