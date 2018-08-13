// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a queue ring size.
///
/// Also known as the number of descriptors, such as `nb_rx_desc` for the number of receive descriptors.
pub trait QueueRingSize: Display + TryFrom<u16> + TryFrom<usize> + Into<u16> + Into<usize>
{
	/// Maximum.
	const Maximum: usize = ::std::u16::MAX as usize;
	
	/// Inclusive Maximum
	const InclusiveMaximum: Self;
}
