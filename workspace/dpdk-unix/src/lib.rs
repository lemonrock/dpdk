// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(int_to_from_bytes)]


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
#[macro_use] extern crate serde_derive;
extern crate rust_extra;
#[cfg(unix)] extern crate syscall_alt;


use self::memory_information::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::mounts::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::linux_kernel_modules::*;
#[cfg(target_os = "linux")] use ::const_cstr_fork::ConstCStr;
use ::libc::*;
use ::std::collections::BTreeSet;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::env::set_var;
use ::std::env::var_os;
use ::std::error::Error;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::NulError;
use ::std::ffi::OsStr;
use ::std::fmt::Display;
use ::std::fs::File;
use ::std::fs::metadata;
use ::std::fs::OpenOptions;
use ::std::fs::Permissions;
use ::std::fs::read_to_string;
use ::std::fs::remove_file;
use ::std::fs::set_permissions;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::ErrorKind;
use ::std::io::Write;
use ::std::num::ParseIntError;
#[cfg(unix)] use ::std::os::unix::io::AsRawFd;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
#[cfg(unix)] use ::std::os::unix::fs::PermissionsExt;
#[cfg(unix)] use ::std::process;
#[cfg(target_os = "linux")] use ::libc_extra::android_linux::stdio::cookie_io_functions_t;
#[cfg(target_os = "linux")] use ::libc_extra::android_linux::stdio::cookie_write_function_t;
#[cfg(target_os = "linux")] use ::libc_extra::android_linux::stdio::fopencookie;
#[cfg(target_os = "linux")] use ::libc_extra::linux::errno::program_invocation_short_name;
#[cfg(target_os = "linux")] use ::libc_extra::unix::stdio::stderr;
#[cfg(target_os = "linux")] use ::libc_extra::unix::stdio::stdout;
#[cfg(unix)] use ::libc_extra::unix::unistd::setegid;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::NonNull;
#[cfg(target_os = "linux")] use ::std::ptr::null_mut;
use ::std::str::FromStr;


#[cfg(any(target_os = "android", target_os = "linux"))]
/// Functionality to provide mid-level wrappers on Linux and Android.
pub mod android_linux;


/// Memory Information.
pub mod memory_information;


/// Support for signals.
#[cfg(unix)] pub mod signals;


pub(crate) mod strings;

include!("assert_effective_user_id_is_root.rs");
include!("Daemonize.rs");
include!("DaemonizeCleanUpOnExit.rs");
include!("get_program_name.rs");
include!("HugePageSize.rs");
include!("HyperThread.rs");
include!("InterruptRequest.rs");
include!("ListParseError.rs");
include!("OsStrExtMore.rs");
#[cfg(unix)] include!("page_size.rs");
include!("PathExt.rs");
include!("ProcPath.rs");
include!("set_current_thread_name.rs");
include!("SetCurrentThreadNameError.rs");
include!("SysPath.rs");
include!("VirtualMemoryStatisticName.rs");
