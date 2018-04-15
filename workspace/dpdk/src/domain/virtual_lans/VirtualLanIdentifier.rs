// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.impl Default for VirtualLanIdentifier


/// Virtual LAN identifier.
///
/// Defaults to one (1).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualLanIdentifier(u16);

impl Serialize for VirtualLanIdentifier
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_u16(self.0)
	}
}

impl Deserialize for VirtualLanIdentifier
{
	#[inline(always)]
	fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error>
	{
		struct U16Visitor;
		
		impl Visitor for U16Visitor
		{
			type Value = u16;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("A Virtual LAN identifier between 1 and 4094 inclusive")
			}
			
			#[inline(always)]
			fn visit_u8<E: de::Error>(self, value: u8) -> Result<Self::Value, E>
			{
				if unlikely(value == 0)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_u16<E: de::Error>(self, value: u16) -> Result<Self::Value, E>
			{
				if unlikely(value == 0)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero".to_string()))
				}
				Ok(value)
			}
			
			#[inline(always)]
			fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E>
			{
				if unlikely(value == 0 || value > 4094)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero or greater than 4094".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E>
			{
				if unlikely(value == 0 || value > 4094)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero or greater than 4094".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i8<E: de::Error>(self, value: i8) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero or negative".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i16<E: de::Error>(self, value: i16) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero or negative".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i32<E: de::Error>(self, value: i32) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0 || value > 4094)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero, negative or greater than 4094".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0 || value > 4094)
				{
					return Err(E::custom("A Virtual LAN identifier can not be zero, negative or greater than 4094".to_string()))
				}
				Ok(value as Self::Value)
			}
		}
		
		let inner_value = deserializer.deserialize_u16(U16Visitor)?;
		Ok(VirtualLanIdentifier(inner_value))
	}
}

impl Default for VirtualLanIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::One
	}
}

impl VirtualLanIdentifier
{
	/// One (1).
	pub const One: Self = VirtualLanIdentifier(1);
	
	/// Extracts a Virtual LAN identifier from tag control information (TCI) if possible.
	#[inline(always)]
	pub fn extract_from_tag_control_information(native_endian_value: u16) -> Result<Option<Self>, ()>
	{
		let identifier = native_endian_value & 0xFFF;
		if unlikely(identifier == 0xFFF)
		{
			return Err(());
		}
		else if unlikely(identifier == 0x000)
		{
			return Ok(None);
		}
		else
		{
			return Ok(Some(VirtualLanIdentifier(identifier)))
		}
	}
	
	/// Value.
	#[inline(always)]
	pub fn value(self) -> u16
	{
		self.0
	}
}
