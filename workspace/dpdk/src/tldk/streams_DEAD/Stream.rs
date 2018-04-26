// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An abstraction of a TCP or UDP stream.
pub trait Stream
{
	/// The layer 4 protocol (TCP or UDP) used by this stream.
	const Protocol: Layer4Protocol;
	
	/// Maximum segment size.
	///
	/// Constant for UDP streams.
	#[inline(always)]
	fn maximum_segment_size(&self) -> u16;
}
