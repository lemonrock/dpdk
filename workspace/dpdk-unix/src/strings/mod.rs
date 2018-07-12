// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::c_char;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::rust_extra::unlikely;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::ffi::CStr;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::ffi::OsStr;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::os::unix::ffi::OsStrExt;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::path::PathBuf;


include!("c_string_pointer_to_path_buf.rs");
include!("c_string_pointer_to_string_with_replacements_if_any.rs");
