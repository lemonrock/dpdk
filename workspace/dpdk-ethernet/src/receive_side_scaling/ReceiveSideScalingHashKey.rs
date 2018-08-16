// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// NOTE: It is tempting to wrap the values in a `Cow`, however, since DPDK often wants a mutable reference to this data and, internally, the `Cow.to_mut()` makes a clone, there is no advantage.
/// A receive side scaling (RSS) hash key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum ReceiveSideScalingHashKey
{
	/// Forty (40) byte key.
	Forty(HashFunctionKeyData40Bytes),
	
	/// Fifty-two (52) byte key.
	FiftyTwo(HashFunctionKeyData52Bytes),
}

impl ReceiveSideScalingHashKey
{
	/// Pointer and length.
	#[inline(always)]
	pub fn pointer_and_length(&mut self) -> (*mut u8, u8)
	{
		use self::ReceiveSideScalingHashKey::*;
		
		match *self
		{
			Forty(ref mut forty_bytes) => (forty_bytes.0.as_mut_ptr(), 40),
			
			FiftyTwo(ref mut fifty_two_bytes) => (fifty_two_bytes.0.as_mut_ptr(), 52),
		}
	}
}
