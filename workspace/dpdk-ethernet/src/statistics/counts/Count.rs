// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A count of units, such as bytes or packets.
pub trait Count : Default + Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + From<u64> + From<i64> + Into<u64> + Into<i64> + Display + Sub<Output=Self>
{
	/// Some ethernet devices do not support some simple statistics; they record these as zero, rather than use a sentinel or Option.
	const ZeroOrSimpleStatisticNotSupportedByEthernetDevice: Self;
	
	/// Is zero?
	#[inline(always)]
	fn is_zero(self) -> bool;
}
