// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::libc::c_ulong;
use ::libc::prctl;
use ::libc_extra::android_linux::linux::seccomp::SECCOMP_MODE_STRICT;
use ::libc_extra::android_linux::linux::securebits::SECBIT_NOROOT;
use ::libc_extra::android_linux::linux::securebits::SECBIT_NOROOT_LOCKED;
use ::libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP;
use ::libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP_LOCKED;
use ::libc_extra::android_linux::linux::securebits::SECBIT_KEEP_CAPS_LOCKED;
use ::libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE;
use ::libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED;
use ::libc_extra::android_linux::sys::prctl::PR_SET_DUMPABLE;
use ::libc_extra::android_linux::sys::prctl::PR_SET_NO_NEW_PRIVS;
use ::libc_extra::android_linux::sys::prctl::PR_TASK_PERF_EVENTS_ENABLE;
use ::libc_extra::android_linux::sys::prctl::PR_TASK_PERF_EVENTS_DISABLE;
use ::libc_extra::android_linux::sys::prctl::PR_SET_THP_DISABLE;
use ::libc_extra::android_linux::sys::prctl::PR_SET_TSC;
use ::libc_extra::android_linux::sys::prctl::PR_TSC_ENABLE;
use ::libc_extra::android_linux::sys::prctl::PR_SET_SECCOMP;
use ::libc_extra::android_linux::sys::prctl::PR_SET_SECUREBITS;


include!("adjust_performance_events.rs");
include!("adjust_transparent_huge_pages.rs");
include!("disable_dumpable.rs");
include!("enable_strict_sec_comp.rs");
include!("enable_tsc_clock.rs");
include!("lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities.rs");
include!("no_new_privileges.rs");
