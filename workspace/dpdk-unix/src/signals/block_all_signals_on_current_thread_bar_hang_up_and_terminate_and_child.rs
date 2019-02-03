// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Block all signals on the current thread bar `SIGUP`, `SIGTERM` and `SIGCHLD` (ie a child process has exited).
#[inline(always)]
pub fn block_all_signals_on_current_thread_bar_hang_up_and_terminate_and_child()
{
	let mut signals = HashSet::with_capacity(3);
	signals.insert(SIGHUP);
	signals.insert(SIGTERM);
	signals.insert(SIGCHLD);
	block_all_signals_on_current_thread_bar(&signals)
}
