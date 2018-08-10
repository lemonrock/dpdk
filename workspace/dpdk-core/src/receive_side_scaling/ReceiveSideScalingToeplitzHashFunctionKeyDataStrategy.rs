// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive side scaling toeplitz hash function key data strategy.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	/// Use fixed values.
	Fixed(ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes, ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes),
	
	/// Generate a Layer 4 hash key using the number of queues as an input.
	ForNumberOfQueues,
}

impl Default for ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	#[inline(always)]
	fn default() -> Self
	{
		ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy::ForNumberOfQueues
	}
}

impl ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy
{
	/// Creates an array of receive side scaling bytes.
	#[inline(always)]
	pub fn create(&self, hash_key_size: u8, number_of_receive_queues: u16) -> Either<Cow<ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes>, Cow<ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes>>
	{
		use self::ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy::*;
		use self::Cow::*;
		use self::Either::*;
		
		const SomePollModeDriversSuchAsMellanox5ReportZeroInsteadOfForty: u8 = 0;
		const Length40: u8 = 40;
		const Length52: u8 = 52;
		
		match *self
		{
			Fixed(ref key_data_40_bytes, ref key_data_52_bytes) =>
			{
				match hash_key_size
				{
					SomePollModeDriversSuchAsMellanox5ReportZeroInsteadOfForty | Length40 => Left(Borrowed(key_data_40_bytes)),
					
					Length52 => Right(Borrowed(key_data_52_bytes)),
					
					_ => panic!("Invalid hash_key_size, '{}'", hash_key_size),
				}
			}
			
			ForNumberOfQueues =>
			{
				match hash_key_size
				{
					SomePollModeDriversSuchAsMellanox5ReportZeroInsteadOfForty | Length40 => Left(Owned(ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes::for_layer_4_one_way_for_number_of_queues(number_of_receive_queues))),

					Length52 => Right(Owned(ReceiveSideScalingToeplitzHashFunctionKeyData52Bytes::for_layer_4_one_way_for_number_of_queues(number_of_receive_queues))),

					_ => panic!("Invalid hash_key_size, '{}'", hash_key_size),
				}
			}
		}
	}
}
