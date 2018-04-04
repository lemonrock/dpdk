// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



pub trait Lifecycle
{
	// RTE_RING_NAMESIZE
	const MaximumNameSizeIncludingFinalAsciiNul: usize = 32;

	#[inline(always)]
	fn free(ring: *mut rte_ring);
		
	#[inline(always)]
	fn guardMaximumNameLength(name: &str)
	{
		debug_assert!(name.len() + 1 <= Self::MaximumNameSizeIncludingFinalAsciiNul, "name '{}', with a final ASCII NUL, would be longer than the maximum of '{}'", name, Self::MaximumNameSizeIncludingFinalAsciiNul);
	}
}
