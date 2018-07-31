// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Converts a hash set of signals to a libc `sigset_t`.
#[inline(always)]
pub fn hash_set_to_signal_set(signals: &HashSet<i32>) -> sigset_t
{
	unsafe
	{
		let mut signal_set: sigset_t = uninitialized();
		sigemptyset(&mut signal_set);
		for signal in signals.iter()
		{
			sigaddset(&mut signal_set, *signal);
		}
		signal_set
	}
}
