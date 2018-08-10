// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive side scaling toeplitz hash function key data (52 byte variants for Intel i40e).
pub struct ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes([u8; ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes::Size]);

impl<'deserialize> Deserialize<'deserialize> for ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		const Size: usize = ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes::Size;
		
		struct DeserializeVisitor;
		
		impl<'deserialize> Visitor<'deserialize> for DeserializeVisitor
		{
			type Value = ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				write!(formatter, "a {} byte array", Size)
			}
			
			#[inline(always)]
			fn visit_bytes<E: DeserializeError>(self, v: &[u8]) -> Result<Self::Value, E>
			{
				if v.len() != Size
				{
					return Err(E::invalid_length(v.len(), &self))
				}
				
				let mut result = ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes(unsafe { uninitialized() });
				result.0.as_mut().clone_from_slice(v);
				Ok(result)
			}
			
			#[inline(always)]
			fn visit_seq<A: SeqAccess<'deserialize>>(self, mut access: A) -> Result<Self::Value, A::Error>
			{
				let mut result = ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes(unsafe { uninitialized() });
				
				// Visit each element in the inner array and push it onto
				// the existing vector.
				let mut index = 0;
				while let Some(byte) = access.next_element()?
				{
					if index == Size
					{
						return Err(A::Error::invalid_length(index, &self))
					}
					* (unsafe { result.0.get_unchecked_mut(index) }) = byte;
					index += 1;
				}
				if index != Size
				{
					Err(A::Error::invalid_length(index, &self))
				}
				else
				{
					Ok(result)
				}
			}
		}
		
		deserializer.deserialize_tuple(Size, DeserializeVisitor)
	}
}

impl Serialize for ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		const Size: usize = ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes::Size;
		
		let mut tuple = serializer.serialize_tuple(Size)?;
		for index in 0 .. Size
		{
			tuple.serialize_element(unsafe { self.0.get_unchecked(index) })?;
		}
		tuple.end()
	}
}

impl ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
{
	/// Size.
	pub const Size: usize = 52;
	
	/// Intel i40e default key.
	pub const IntelI40EDefault: Self = ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
	([
		0x44, 0x39, 0x79, 0x6B, 0xB5, 0x4C, 0x50, 0x23,
		0xB6, 0x75, 0xEA, 0x5B, 0x12, 0x4F, 0x9F, 0x30,
		0xB8, 0xA2, 0xC0, 0x3D, 0xDF, 0xDC, 0x4D, 0x02,
		0xA0, 0x8C, 0x9B, 0x33, 0x4A, 0xF6, 0x4A, 0x4C,
		0x05, 0xC6, 0xFA, 0x34, 0x39, 0x58, 0xD8, 0x55,
		0x7D, 0x99, 0x58, 0x3A, 0xE1, 0x38, 0xC9, 0x2E,
		0x81, 0x15, 0x03, 0x66,
	]);
	
	/// Guessed.
	#[inline(always)]
	pub fn for_layer_4_one_way_for_number_of_queues(number_of_receive_queues: u16) -> Self
	{
		let variable_byte = (number_of_receive_queues.next_power_of_two() & 0xFF) as u8;
		
		ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
		([
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, variable_byte,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, variable_byte,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00,
		])
	}
	
	/// To a vector of bytes.
	#[inline(always)]
	pub fn to_vec(&self) -> Vec<u8>
	{
		(&self.0[..]).to_vec()
	}
}
