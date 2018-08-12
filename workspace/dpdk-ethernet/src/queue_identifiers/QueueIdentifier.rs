// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a queue identifier.
pub trait QueueIdentifier: TryFrom<u16> + TryFrom<usize> + Into<u16> + Into<usize> +  Step + Add<u16, Output=Self> + Add<usize, Output=Self> + AddAssign<u16> + AddAssign<usize> + Sub<u16, Output=Self> + Sub<usize, Output=Self> + SubAssign<u16> + SubAssign<usize>
{
	/// Maximum.
	const Maximum: usize = RTE_MAX_QUEUES_PER_PORT as usize;
	
	/// Zero.
	///
	/// Smallest possible queue identifier.
	const Zero: Self;
	
	/// Largest possible queue identifier.
	const InclusiveMaximum: Self;
}
