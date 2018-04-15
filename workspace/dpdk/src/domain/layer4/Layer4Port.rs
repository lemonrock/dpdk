// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A layer 4 port number (eg 80 for HTTP over TCP).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layer4Port(u16);

impl Serialize for Layer4Port
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_u16(self.0)
	}
}

impl Deserialize for Layer4Port
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
				formatter.write_str("A Layer 4 port number suitable for TCP, UDP and SCTP between 1 and 4094 inclusive")
			}
			
			#[inline(always)]
			fn visit_u8<E: de::Error>(self, value: u8) -> Result<Self::Value, E>
			{
				if unlikely(value == 0)
				{
					return Err(E::custom("A layer 4 port can not be zero".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_u16<E: de::Error>(self, value: u16) -> Result<Self::Value, E>
			{
				if unlikely(value == 0)
				{
					return Err(E::custom("A layer 4 port can not be zero".to_string()))
				}
				Ok(value)
			}
			
			#[inline(always)]
			fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E>
			{
				if unlikely(value == 0 || value > 65_535)
				{
					return Err(E::custom("A layer 4 port can not be zero or greater than 65535".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E>
			{
				if unlikely(value == 0 || value > 65_535)
				{
					return Err(E::custom("A layer 4 port can not be zero or greater than 65535".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i8<E: de::Error>(self, value: i8) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0)
				{
					return Err(E::custom("A layer 4 port can not be zero or negative".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i16<E: de::Error>(self, value: i16) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0)
				{
					return Err(E::custom("A layer 4 port can not be zero or negative".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i32<E: de::Error>(self, value: i32) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0 || value > 65_535)
				{
					return Err(E::custom("A layer 4 port can not be zero, negative or greater than 65535".to_string()))
				}
				Ok(value as Self::Value)
			}
			
			#[inline(always)]
			fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E>
			{
				if unlikely(value <= 0 || value > 65_535)
				{
					return Err(E::custom("A layer 4 port can not be zero, negative or greater than 65535".to_string()))
				}
				Ok(value as Self::Value)
			}
		}
		
		let innerValue = deserializer.deserialize_u16(U16Visitor)?;
		Ok(Layer4Port(innerValue))
	}
}

impl Layer4Port
{
	/// HTTP over TCP port number.
	pub const Http: Layer4Port = Layer4Port(80);
	
	/// HTTPS over TCP port number.
	pub const Https: Layer4Port = Layer4Port(443);
	
	#[inline(always)]
	pub fn convert_from_tcp_or_udp_or_sctp_port_value_in_layer_4_header(native_endian_value: u16) -> Result<Self, ()>
	{
		if unlikely(native_endian_value == 0)
		{
			Err(())
		}
		else
		{
			Ok(Layer4Port(native_endian_value))
		}
	}
}
