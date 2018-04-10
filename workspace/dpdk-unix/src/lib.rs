// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]


//! #dpdk-unix
//!
//! This crate proves additional mid-level functionality for Unix-like Operating Systems which wraps functionality found in low-level FFI bindings for libc.
//!
//! It also provides a very small modicum of Windows support to get the current program name.


#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate bitflags;
#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate const_cstr_fork;
extern crate errno;
extern crate libc;
extern crate libc_extra;
#[cfg(unix)] #[macro_use] extern crate maplit;
#[macro_use] extern crate quick_error;
#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate serde_derive;
extern crate rust_extra;
#[cfg(unix)] extern crate syscall_alt;


use ::libc::geteuid;
use ::std::fmt::Display;
use ::std::error::Error;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::NulError;
use ::std::ffi::OsStr;
use ::std::fs::File;
use ::std::fs::metadata;
use ::std::fs::OpenOptions;
use ::std::fs::Permissions;
use ::std::fs::set_permissions;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::Read;
use ::std::io::Write;
use ::std::num::ParseIntError;
#[cfg(unix)] use ::std::os::unix::fs::PermissionsExt;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
use ::std::path::Path;
use ::std::str::FromStr;


#[cfg(any(target_os = "android", target_os = "linux"))]
/// Functionality to provide mid-level wrappers on Linux and Android.
pub mod android_linux;


/// Support for signals.
#[cfg(unix)] pub mod signals;


pub(crate) mod strings;


include!("assert_effective_user_id_is_root.rs");
include!("get_program_name.rs");
include!("HugePageSize.rs");
include!("OsStrExtMore.rs");
include!("PathExt.rs");
include!("set_current_thread_name.rs");
include!("SetCurrentThreadNameError.rs");
