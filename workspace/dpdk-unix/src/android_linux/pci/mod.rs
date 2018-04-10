// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(unix)] use ::errno::errno;
#[cfg(unix)] use ::libc::AF_INET;
#[cfg(unix)] use ::libc::c_void;
#[cfg(unix)] use ::libc::close;
#[cfg(unix)] use ::libc::ioctl;
#[cfg(unix)] use ::libc::IPPROTO_IP;
#[cfg(unix)] use ::libc::SOCK_DGRAM;
#[cfg(unix)] use ::libc::socket;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::net::if_::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::linux::ethtool::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::linux::sockios::SIOCETHTOOL;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::ffi::CStr;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::mem::transmute;
#[cfg(unix)] use ::std::os::unix::io::RawFd;
#[cfg(unix)] use ::std::ptr::write;
#[cfg(unix)] use ::syscall_alt::constants::E;


#[cfg(unix)] include!("OpenPciBusInformationError.rs");
#[cfg(unix)] include!("PciBusInformation.rs");


