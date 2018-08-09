// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This macro isn't perfect.
///
/// It could be made even simpler by using a macro to parse a struct's fields, and extract only those that are public.
/// This is very much what the `parse-macros` crate does.
/// However, such logic is *extremely* complex to both write, understand and maintain.
macro_rules! custom_deserialize
{
	(
		$type: tt,
		$(
			$field_index: expr => $field_name: tt,
		)*
	) =>
	{
		macro_rules! sequence_field
		{
			($self: ident, $access: ident, $field_index_inner: expr) =>
			{
				$access.next_element()?.ok_or_else(|| DeserializerError::invalid_length($field_index_inner, &$self))?
			}
		}
		
		macro_rules! map_field
		{
			($field_name_inner: ident) =>
			{
				$field_name_inner.ok_or_else(|| DeserializerError::missing_field(stringify!($field_name_inner)))?
			}
		}
		
		impl<'deserialize> Deserialize<'deserialize> for $type
		{
			#[inline(always)]
			fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
			{
				struct DeserializingVisitor;
				
				impl<'deserialize> Visitor<'deserialize> for DeserializingVisitor
				{
					type Value = $type;
					
					#[inline(always)]
					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
					{
						formatter.write_str(stringify!($type))
					}
					
					#[inline(always)]
					fn visit_seq<V: SeqAccess<'deserialize>>(self, mut access: V) -> Result<Self::Value, V::Error>
					{
						Ok
						(
							$type::new
							(
								$(
									sequence_field!(self, access, $field_index),
								)*
							)
						)
					}
					
					#[inline(always)]
					fn visit_map<V: MapAccess<'deserialize>>(self, mut access: V) -> Result<Self::Value, V::Error>
					{
						#[allow(non_camel_case_types)]
						#[derive(Deserialize)]
						enum Field
						{
							$(
								$field_name,
							)*
						}
						
						$(
							let mut $field_name = None;
						)*
						
						while let Some(key) = access.next_key()?
						{
							match key
							{
								$(
									Field::$field_name =>
									{
										if $field_name.is_some()
										{
											return Err(DeserializerError::duplicate_field(stringify!($field_name)));
										}
										$field_name = Some(access.next_value()?);
									}
								)*
							}
						}
						
						Ok
						(
							$type::new
							(
								$(
									map_field!($field_name),
								)*
							)
						)
					}
				}
				
				deserializer.deserialize_struct
				(
					stringify!($type),
					&[
						$(
							stringify!($field_name),
						)*
					],
					DeserializingVisitor
				)
			}
		}
	}
}
