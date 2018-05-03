// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate arrayvec;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate const_cstr_fork;
pub extern crate pointer;
#[macro_use] pub extern crate dpdk_serde;
pub extern crate dpdk_sys;
#[cfg(unix)] pub extern crate dpdk_unix;
extern crate hyper_thread_random;
#[macro_use] extern crate lazy_static;
extern crate libc;
extern crate libc_extra;
extern crate libnuma_sys;
#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;
extern crate rust_extra;
#[macro_use] extern crate stderr_logging;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate syscall_alt;
extern crate treebitmap;


use self::*;
use self::bus::*;
use self::bus::pci::*;
use self::bus::pci::subclasses::*;
use self::devices::*;
use self::devices::ethernet::*;
use self::devices::virtual_devices::*;
use self::devices::virtual_devices::net_virtual_devices::*;
use self::domain::*;
use self::domain::address_resolution_protocol::*;
use self::domain::ethernet::*;
use self::domain::internet_protocol::*;
use self::domain::internet_protocol::longest_prefix_matching::*;
use self::domain::internet_protocol::mask_bits::*;
use self::domain::internet_protocol::packet_fragmentation::*;
use self::domain::internet_protocol::routing::*;
use self::domain::layer4::*;
use self::domain::layer4::internet_control_message_protocol::*;
use self::domain::virtual_lans::*;
use self::E_RTE::*;
use self::ethernetPorts::*;
use self::internet_protocol_packet_reassembly::*;
use self::logicalCores::*;
use self::logicalCores::active::*;
use self::logicalCores::discovery::*;
use self::logicalCores::receiveTransmitQueuePair::*;
use self::memory::*;
use self::memory::zones::*;
use self::packet_buffers::*;
use self::packet_buffers::packet_types::*;
use self::power::*;
use self::print_information::*;
use self::process::*;
use self::time::*;
use self::tldk::*;
use self::tldk::devices::*;
use ::arrayvec::ArrayVec;
use ::const_cstr_fork::ConstCStr;
use ::dpdk_unix::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::android_linux::pci::PciBusInformation;
use ::dpdk_sys::*;
use ::hyper_thread_random::generate_hyper_thread_safe_random_u64;
use ::indexmap::set::IndexSet;
use ::libc::*;
use ::libc_extra::*;
use ::libc_extra::ffi::*;
use ::libc_extra::ffi::arguments::*;
use ::libc_extra::stdio::open_memstream;
use ::libnuma_sys::*;
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
use ::std::cell::RefCell;
use ::std::cell::UnsafeCell;
use ::std::cmp::max;
use ::std::cmp::min;
use ::std::cmp::Ord;
use ::std::cmp::Ordering;
use ::std::cmp::PartialOrd;
use ::std::collections::BTreeSet;
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
use ::std::fs::File;
use ::std::marker::PhantomData;
use ::std::hash::Hash;
use ::std::io;
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
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::std::str::SplitN;
use ::std::string::FromUtf8Error;
use ::std::sync::Arc;
use ::std::sync::Mutex;
use ::std::sync::Once;
use ::std::sync::ONCE_INIT;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::SeqCst;
use ::std::thread::sleep;
use ::std::time::Duration;
use ::syscall_alt::PosixErrorNumber;
use ::syscall_alt::constants::NegativeE;
use ::syscall_alt::constants::E;
use ::treebitmap::*;


/// DPDK devices.
pub mod devices;


/// Domain-like structs for ARP, Ethernet, Internet Protocol, Layer 2 and virtual LANs.
pub mod domain;


pub(crate) mod E_RTE;


/// Ethernet ports.
pub mod ethernetPorts;


/// Logical cores and NUMA nodes.
pub mod logicalCores;


/// DPDK memory management.
pub mod memory;


/// PCI devices.
pub mod pci;


/// Packet buffers.
#[macro_use] pub mod packet_buffers;


/// CPU power management.
pub mod power;


/// Process and configuration helpers.
pub mod process;


/// Print information helpers.
#[cfg(target_os = "linux")] pub mod print_information;


/// Alarms and Timers.
pub mod time;


/// Layer 4 (TLDK).
pub mod tldk;


include!("finish.rs");


include!("AllLogicalCoreIterator.rs");
include!("LogicalCore.rs");
include!("LogicalCoreChoice.rs");
include!("NumaNode.rs");
include!("NumaNodeChoice.rs");
include!("PointerExt.rs");
include!("Service.rs");
include!("ServiceFunction.rs");
include!("ShouldFunctionTerminate.rs");
include!("SlaveLogicalCoreFunction.rs");
include!("SlaveLogicalCoreIterator.rs");
