// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))] use ::errno::errno;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::syscall_alt::constants::E;
use ::libc::*;
use ::std::collections::HashSet;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;


include!("block_all_signals_on_current_thread.rs");
include!("block_all_signals_on_current_thread_bar.rs");
include!("block_all_signals_on_current_thread_bar_child.rs");
include!("block_all_signals_on_current_thread_bar_hang_up_and_terminate_and_child.rs");
include!("hash_set_to_signal_set.rs");
include!("one_millisecond_timed_wait_for_signals.rs");
include!("SignalNumber.rs");
include!("TimedSignalWait.rs");
