// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(associated_consts)]
#![feature(const_fn)]


#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate bitflags_associated_constants;
#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate const_cstr_fork;
extern crate errno;
extern crate libc;
extern crate libc_extra;
#[cfg(unix)] extern crate nix;
#[macro_use] extern crate quick_error;
#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate serde_derive;
extern crate rust_extra;
#[cfg(unix)] extern crate syscall_alt;


#[cfg(any(target_os = "android", target_os = "linux"))] pub mod android_linux;
pub mod helpers;
pub mod strings;


include!("HugePageSize.rs");
