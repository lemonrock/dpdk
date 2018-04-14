// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate arrayvec;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate const_cstr_fork;
pub extern crate pointer;
#[macro_use] pub extern crate dpdk_serde;
pub extern crate dpdk_sys;
#[cfg(unix)] pub extern crate dpdk_unix;
extern crate libc;
extern crate libc_extra;
#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;
extern crate rust_extra;
#[macro_use] extern crate stderr_logging;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate syscall_alt;


use ::arrayvec::ArrayVec;
use ::const_cstr_fork::ConstCStr;
use ::dpdk_unix::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::android_linux::pci::PciBusInformation;
use ::dpdk_sys::*;
use ::libc::*;
use ::libc_extra::*;
use ::rust_extra::arrays::*;
use ::libc_extra::ffi::*;
use ::libc_extra::ffi::arguments::*;
use ::rust_extra::powersOfTwo::*;
use ::rust_extra::*;
use ::serde::*;
use ::serde::de;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::de::Error as DeserializerError;
use ::serde::ser;
use ::serde::ser::Serialize;
use ::serde::ser::Serializer;
use ::std::cell::RefCell;
use ::std::cell::UnsafeCell;
use ::std::cmp::min;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::OsStr;
use ::std::ffi::OsString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::hash::Hash;
use ::std::io;
use ::std::iter::FromIterator;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::size_of_val;
use ::std::mem::transmute;
use ::std::mem::transmute_copy;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::net::IpAddr;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::num::ParseIntError;
use ::std::os::unix::ffi::OsStrExt;
use ::std::panic::catch_unwind;
use ::std::panic::AssertUnwindSafe;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::copy;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::rc::Rc;
use ::std::slice::from_raw_parts_mut;
use ::std::str::SplitN;
use ::std::string::FromUtf8Error;
use ::std::sync::Arc;
use ::std::sync::Mutex;
use ::syscall_alt::constants::NegativeE;
use ::syscall_alt::constants::E;


pub mod devices;


pub mod domain;


pub mod E_RTE;


pub mod ethernetPorts;


pub mod ipFragmentation;


pub mod logicalCores;


pub mod longestPrefixMatch;


pub mod memory;


pub mod memoryZones;


pub mod pci;


#[macro_use] pub mod packetBuffers;


pub mod power;


pub mod process;


/// Alarms and Timers.
pub mod time;


pub mod tldk;

