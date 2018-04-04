// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::rust_rte_errno;
use ::dpdk_sys::rust_rte_reset_errno;
use ::dpdk_sys::rte_timer;
use ::dpdk_sys::rte_timer_cb_t;
use ::dpdk_sys::rte_timer_status;
use ::dpdk_sys::rte_timer_type;
use ::libc::c_void;
use ::libc::FILE;
use ::libc_extra::stderr;
use ::rust_extra::likely;
use ::std::mem::forget;
use ::std::ptr::null_mut;
use ::syscall_alt::constants::E;
use ::libc_extra::ffi::callbacks::MutableCallback1;
use ::libc_extra::ffi::isTrue;
use ::logicalCores::LogicalCore;
use ::time::cycles::getRdtsc;


include!("Alarm.rs");
include!("Timer.rs");
include!("TimerStatus.rs");
include!("TimerCallback.rs");
include!("TimerManager.rs");


pub mod cycles;

