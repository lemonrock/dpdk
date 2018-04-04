// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![feature(allocator)]
#![no_std]


#![allocator]


#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] include!("lib.supported-platforms.rs");
