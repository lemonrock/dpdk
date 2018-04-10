// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(const_fn)]
#![feature(i128_type)]
#![cfg_attr(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")), feature(integer_atomics))]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]


extern crate arrayvec;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] extern crate bitflags;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] extern crate const_cstr_fork;
pub extern crate pointer;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] pub extern crate dpdk_serde;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub extern crate dpdk_sys;
#[cfg(unix)] pub extern crate dpdk_unix;
#[cfg(unix)] extern crate libc;
#[cfg(unix)] extern crate libc_extra;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] extern crate log;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] extern crate quick_error;
extern crate rust_extra;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[cfg(unix)] #[macro_use] extern crate stderr_logging;
extern crate serde;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] extern crate serde_derive;
#[cfg(unix)] extern crate syscall_alt;


#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod devices;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod distributor;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod domain;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod E_RTE;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod ethernetPorts;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod ipFragmentation;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod logicalCores;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod longestPrefixMatch;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod memory;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod memoryZones;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod pci;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] pub mod packetBuffers;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod power;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod process;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod tldk;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod time;

