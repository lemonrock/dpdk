// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_export]
macro_rules! serde_pub_enum_u8
{
	($name:ident { $($variant:ident = $value:expr, )* }) =>
	{
		#[repr(u8)]
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub enum $name
		{
			$($variant = $value,)*
		}

		impl ::serde::Serialize for $name
		{
			#[inline(always)]
			fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
			{
				serializer.serialize_u8(*self as u8)
			}
		}

		impl ::serde::Deserialize for $name
		{
			#[inline(always)]
			fn deserialize<D: ::serde::Deserializer>(deserializer: D) -> Result<Self, D::Error>
			{
				struct Visitor;

				impl ::serde::de::Visitor for Visitor
				{
					type Value = $name;

					fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
					{
						formatter.write_str("positive integer")
					}

					fn visit_u8<E: ::serde::de::Error>(self, value: u8) -> Result<$name, E>
					{
						// Rust does not come with a simple way of converting a number to an enum, so use a big `match`.
						match value
						{
							$( $value => Ok($name::$variant), )*
							_ => Err(E::custom(format!("unknown {} value: {}", stringify!($name), value))),
						}
					}
				}

				deserializer.deserialize_u8(Visitor)
			}
		}
	}
}
