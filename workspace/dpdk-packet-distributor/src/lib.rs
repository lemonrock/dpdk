// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![cfg_attr(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")), feature(core_intrinsics))]


//! #dpdk-global-allocator
//! A wrapper around DPDK's packet distributor framework with support for slave logical cores and service logical cores.
//!
//! An user of this crate should create an instance of `PacketDistributorControllerLoop` and then use DPDK's slave logical cores or service logical cores to run it repeatedly. This particular loop implementation distributes packets to workers ***but does not*** process returned packets.
//!
//! DPDK's underlying code never reserves but never frees memory on creation... potentially limiting the usefulness of this framework.
//!
//! Whilst DPDK is supported for Linux on AArch64, ARM v7, PowerPC 64-bit (recent) and x86-64, and FreeBSD on x86-64, only Linux x86-64 will compile.
//!
//! Some work has been done to try to maintain compatibility with FreeBSD and other architectures but this is not maintained yet.


#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] include!("lib.cfg.rs");
