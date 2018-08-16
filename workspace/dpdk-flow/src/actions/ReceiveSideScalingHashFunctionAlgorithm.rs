// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Depth (in tunnel layers) into a packet to go for its Receive Side Scaling (RSS) hash calculation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum ReceiveSideScalingHashFunctionAlgorithm
{
	/// Network preferred card default.
	///
	/// Always supported, and nearly always defaults to Toeplitz.
	NetworkCardPreferredDefault,
	
	/// XOR hash function.
	///
	/// Not widely supported.
	SimpleXor,
	
	/// Toeplitz hash function.
	///
	/// Widely supported and the default.
	Toeplitz,
}

impl Into<rte_eth_hash_function> for ReceiveSideScalingHashFunctionAlgorithm
{
	#[inline(always)]
	fn into(self) -> rte_eth_hash_function
	{
		use self::ReceiveSideScalingHashFunctionAlgorithm::*;
		use self::rte_eth_hash_function::*;
		
		match self
		{
			NetworkCardPreferredDefault => RTE_ETH_HASH_FUNCTION_DEFAULT,
			
			SimpleXor => RTE_ETH_HASH_FUNCTION_SIMPLE_XOR,
			
			Toeplitz => RTE_ETH_HASH_FUNCTION_TOEPLITZ,
		}
	}
}

impl Default for ReceiveSideScalingHashFunctionAlgorithm
{
	#[inline(always)]
	fn default() -> Self
	{
		ReceiveSideScalingHashFunctionAlgorithm::Toeplitz
	}
}
