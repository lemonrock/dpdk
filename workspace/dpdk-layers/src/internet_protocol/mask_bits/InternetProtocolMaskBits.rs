// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask bits.
pub trait InternetProtocolMaskBits: Sized + Debug + Display + PartialOrd + Ord + PartialEq + Eq + Hash + Serialize + Clone + Copy
{
	#[inline(always)]
	fn as_depth(&self) -> u8;
}
