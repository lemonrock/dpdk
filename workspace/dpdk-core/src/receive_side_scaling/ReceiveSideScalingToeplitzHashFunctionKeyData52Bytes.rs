// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive side scaling toeplitz hash function key data (52 byte variants for Intel i40e).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes(Array52<u8>);

impl ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
{
	/// Intel i40e default key.
	pub const IntelI40EDefault: Self = ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
	(
		Array52
		(
			[
				0x44, 0x39, 0x79, 0x6B, 0xB5, 0x4C, 0x50, 0x23,
				0xB6, 0x75, 0xEA, 0x5B, 0x12, 0x4F, 0x9F, 0x30,
				0xB8, 0xA2, 0xC0, 0x3D, 0xDF, 0xDC, 0x4D, 0x02,
				0xA0, 0x8C, 0x9B, 0x33, 0x4A, 0xF6, 0x4A, 0x4C,
				0x05, 0xC6, 0xFA, 0x34, 0x39, 0x58, 0xD8, 0x55,
				0x7D, 0x99, 0x58, 0x3A, 0xE1, 0x38, 0xC9, 0x2E,
				0x81, 0x15, 0x03, 0x66,
			]
		)
	);
	
	/// Guessed.
	#[inline(always)]
	pub fn for_layer_4_one_way_for_number_of_queues(number_of_receive_queues: u16) -> Self
	{
		let variable_byte = (number_of_receive_queues.next_power_of_two() & 0xFF) as u8;
		
		ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes
		(
			Array52
			(
				
				[
					0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, variable_byte,
					0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, variable_byte,
					0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					0x00, 0x00, 0x00, 0x00,
				]
			)
		)
	}
	
	/// To a vector of bytes.
	#[inline(always)]
	pub fn to_vec(&self) -> Vec<u8>
	{
		(&self.0[..]).to_vec()
	}
}
