// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![feature(static_nobundle)]


#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u8;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u16;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u32;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u64;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__s32;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::cpu_set_t;
#[cfg(target_os = "freebsd")] use ::libc::cpuset_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::FILE;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::in_addr;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::in6_addr;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::int16_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::iovec;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::off_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::pthread_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::sockaddr_storage;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::sockaddr;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::size_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint8_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint16_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint32_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint64_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::timespec;


#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] include!("bindgen/lib.rs");


pub type MARKER8 = uint8_t;

pub type MARKER64 = uint64_t;


#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct __m128i
{
	a: u64,
	b: u64,
}
