// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(renamed_and_removed_lints)]
#![deny(missing_docs)]
//#![cfg_attr(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")), feature(const_fn, core_intrinsics))]


//! #dpdk-virtual-devices
//! Mid-level wrappers around the DPDK virtual devices framework.
//!
//! Whilst DPDK is supported for Linux on AArch64, ARM v7, PowerPC 64-bit (recent) and x86-64, and FreeBSD on x86-64, only Linux x86-64 will compile.
//!
//! Some work has been done to try to maintain compatibility with FreeBSD and other architectures but this is not maintained yet.


#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] include!("lib.cfg.rs");
