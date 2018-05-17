// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![cfg_attr(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")), feature(const_fn, global_allocator))]
#![warn(missing_docs)]


#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] include!("main.cfg.rs");


#[cfg(not(any(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64"))))]
fn main()
{
	eprintln!("This program is not supported on your platform")
}
