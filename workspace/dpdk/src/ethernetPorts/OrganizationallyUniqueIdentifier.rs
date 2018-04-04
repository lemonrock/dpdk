// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct OrganizationallyUniqueIdentifier(pub [u8; 3]);

impl Display for OrganizationallyUniqueIdentifier
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		let bytes = self.0;
		write!(f, "{:02X}:{:02X}:{:02X}", bytes[0], bytes[1], bytes[2])
	}
}

impl Serialize for OrganizationallyUniqueIdentifier
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_str(&format!("{}", self))
	}
}

impl Deserialize for OrganizationallyUniqueIdentifier
{
	fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error>
	{
		struct FromString;
		
		impl Visitor for FromString
		{
			type Value = OrganizationallyUniqueIdentifier;
			
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				formatter.write_str("string of 3 2-byte hexadecimal values separated by colons, eg 00:AA:BB")
			}
			
			fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E>
			{
				fn next<'a, E: Error>(splits: &mut SplitN<'a, char>) -> Result<u8, E>
				{
					if let Some(hexadecimalByteWithoutPrefix) = splits.next()
					{
						match u8::from_str_radix(hexadecimalByteWithoutPrefix, 16)
						{
							Ok(value) => Ok(value),
							Err(_) => Err(E::custom("Could not convert hexadecimal byte in OrganizationallyUniqueIdentifier")),
						}
					}
					else
					{
						Err(E::custom("Less than 3 hexadecimal bytes in OrganizationallyUniqueIdentifier"))
					}
				}
				
				let splits = &mut value.splitn(6, ':');
				
				let bytes =
				[
					next(splits)?,
					next(splits)?,
					next(splits)?,
				];
				
				if splits.next().is_some()
				{
					Err(E::custom("More than 3 hexadecimal bytes"))
				}
				else
				{
					Ok(OrganizationallyUniqueIdentifier(bytes))
				}
			}
		}
		
		deserializer.deserialize(FromString)
	}
}
