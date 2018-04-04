// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]


extern crate errno;
extern crate libc;
extern crate libc_extra;
extern crate syscall_alt;
#[allow(unused_extern_crates) ]#[macro_use] extern crate quick_error;


use ::errno::errno;
use ::libc::c_int;
use ::libc::AF_INET;
use ::libc::SOCK_DGRAM;
use ::libc::IPPROTO_IP;
use ::libc::socket;
use ::libc::close;
use ::syscall_alt::constants::E;


#[cfg(any(target_os = "android", target_os = "linux"))] pub mod android_linux;


include!("closeSocketFileDescriptor.rs");
include!("OpenIoCtlSocketError.rs");
include!("openSocketForIoCtl.rs");


// Should be XXXX:XX:XX.XX
pub const NumberOfBytesInPciAddressString: usize = 13;
