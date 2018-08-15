// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A receive side scaling (RSS) hash key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ReceiveSideScalingHashKey<'a>(Either<Cow<'a, HashFunctionKeyData40Bytes>, Cow<'a, HashFunctionKeyData52Bytes>>);

impl<'a> ReceiveSideScalingHashKey<'a>
{
	#[inline(always)]
	pub(crate) fn pointer_and_length(&mut self) -> (*mut u8, u8)
	{
		use self::Either::*;
		
		match self.0
		{
			Left(ref mut forty_bytes) => (forty_bytes.to_mut().0.as_mut_ptr(), 40),
			
			Right(ref mut fifty_two_bytes) => (fifty_two_bytes.to_mut().0.as_mut_ptr(), 52),
		}
	}
}
