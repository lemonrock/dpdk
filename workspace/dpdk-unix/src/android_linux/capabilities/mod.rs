// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::errno::errno;
use ::libc::c_long;
use ::libc::c_ulong;
use ::libc::c_void;
use ::libc::getpid;
use ::libc::prctl;
use ::libc::syscall;
use ::libc_extra::android_linux::linux::capability;
use ::libc_extra::android_linux::linux::capability::cap_user_header_t;
use ::libc_extra::android_linux::linux::capability::cap_user_data_t;
use ::libc_extra::android_linux::linux::capability::__user_cap_header_struct;
use ::libc_extra::android_linux::linux::capability::__user_cap_data_struct;
use ::libc_extra::android_linux::linux::capability::_LINUX_CAPABILITY_VERSION_3;
use ::libc_extra::android_linux::sys::prctl::PR_CAPBSET_DROP;
use ::libc_extra::android_linux::sys::prctl::PR_CAPBSET_READ;
use ::libc_extra::android_linux::sys::prctl::PR_CAP_AMBIENT;
use ::libc_extra::android_linux::sys::prctl::PR_CAP_AMBIENT_CLEAR_ALL;
use ::std::mem::zeroed;
use ::syscall_alt::constants::SYS::SYS_capget;
use ::syscall_alt::constants::SYS::SYS_capset;
use ::syscall_alt::constants::E;


include!("Capability.rs");
include!("CapabilitySet.rs");
