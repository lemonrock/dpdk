// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate arrayvec;
#[macro_use] extern crate bitflags;
#[macro_use] pub extern crate dpdk_serde;
pub extern crate dpdk_sys;
#[cfg(unix)] pub extern crate dpdk_unix;
extern crate hashbrown;
extern crate hyper_thread_random;
#[macro_use] extern crate lazy_static;
extern crate libc;
extern crate libc_extra;
extern crate libnuma_sys;
extern crate linked_list_allocator;
extern crate lock_free_multi_producer_single_consumer_ring_buffer;
#[macro_use] extern crate log;
extern crate raw_cpuid;
extern crate rust_extra;
#[macro_use] extern crate stderr_logging;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate syscall_alt;
extern crate treebitmap;


use ::arrayvec::ArrayVec;
use ::dpdk_unix::*;
use ::dpdk_unix::signals::*;
use ::dpdk_unix::strings::*;
#[cfg(target_os = "linux")] use ::dpdk_unix::android_linux::capabilities::*;
#[cfg(target_os = "linux")] use ::dpdk_unix::android_linux::pci::PciBusInformation;
#[cfg(target_os = "linux")] use ::dpdk_unix::android_linux::process_control::*;
use ::dpdk_sys::*;
use ::hashbrown::HashMap;
use ::hashbrown::HashSet;
use ::hyper_thread_random::generate_hyper_thread_safe_random_u64;
use ::indexmap::set::IndexSet;
use ::libc::*;
use ::libc_extra::stdlib::*;
use ::libc_extra::stdio::*;
use ::libnuma_sys::*;
use ::linked_list_allocator::*;
use ::lock_free_multi_producer_single_consumer_ring_buffer::*;
use ::raw_cpuid::*;
use ::rust_extra::*;
use ::rust_extra::arrays::*;
use ::rust_extra::powersOfTwo::*;
use ::serde::*;
use ::serde::de;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::de::Error as DeserializerError;
use ::serde::ser;
use ::serde::ser::Serialize;
use ::serde::ser::Serializer;
use ::std::any::Any;
use ::std::alloc::GlobalAlloc;
use ::std::alloc::Layout;
use ::std::alloc::Opaque;
use ::std::cell::RefCell;
use ::std::cell::UnsafeCell;
use ::std::cmp::max;
use ::std::cmp::min;
use ::std::cmp::Ord;
use ::std::cmp::Ordering;
use ::std::cmp::PartialOrd;
use ::std::collections::BTreeSet;
use ::std::env::current_exe;
use ::std::env::set_var;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::OsStr;
use ::std::ffi::OsString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::marker::PhantomData;
use ::std::hash::Hash;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::ErrorKind;
use ::std::iter::FromIterator;
use ::std::iter::Step;
use ::std::mem::forget;
use ::std::mem::replace;
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
use ::std::os::unix::io::RawFd;
use ::std::os::unix::io::IntoRawFd;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::catch_unwind;
use ::std::panic::resume_unwind;
use ::std::panic::as_initialization_argument_hook;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::process::ExitCode;
use ::std::ptr::copy;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::rc::Rc;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::std::str::SplitN;
use ::std::string::FromUtf8Error;
use ::std::sync::Arc;
use ::std::sync::Mutex;
use ::std::sync::Once;
use ::std::sync::ONCE_INIT;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::AcqRel;
use ::std::sync::atomic::Ordering::Release;
use ::std::sync::atomic::Ordering::SeqCst;
use ::std::sync::atomic::spin_loop_hint;
use ::std::thread::sleep;
use ::std::time::Duration;
use ::syscall_alt::PosixErrorNumber;
use ::syscall_alt::constants::NegativeE;
use ::syscall_alt::constants::E;
use ::treebitmap::*;


/// Ethernet.
pub mod ethernet;


/// Ethernet ports.
pub mod ethernetPorts;


/// Packet buffers.
#[macro_use] pub mod packet_buffers;

