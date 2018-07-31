// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Block all signals specified the current thread.
#[inline(always)]
pub fn block_all_signals_on_current_thread_bar(signals: &HashSet<SignalNumber>)
{
	let result = unsafe
	{
		let mut set = uninitialized();
		sigfillset(&mut set);
		for signal in signals.iter()
		{
			sigdelset(&mut set, *signal);
		}
		pthread_sigmask(SIG_SETMASK, &set, null_mut())
	};
	
	match result
	{
		0 => (),
		-1 => panic!("pthread_sigmask returned an error"),
		_ => panic!("pthread_sigmask returned an invalid result '{}'", result)
	}
}
