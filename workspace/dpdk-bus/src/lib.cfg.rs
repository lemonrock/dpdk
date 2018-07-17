// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_use] extern crate bitflags;
extern crate const_cstr_fork;
extern crate dpdk_core;
#[macro_use] extern crate dpdk_likely;
extern crate dpdk_sys;
extern crate dpdk_unix;
extern crate libc;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate quick_error;


use ::const_cstr_fork::ConstCStr;
use ::dpdk_core::*;
use ::dpdk_sys::*;
use ::dpdk_unix::*;
use ::libc::*;
use ::std::collections::BTreeSet;
use ::std::cell::UnsafeCell;
use ::std::cmp::Ordering;
use ::std::marker::PhantomData;
use ::std::os::unix::io::RawFd;
use ::std::path::PathBuf;
use ::std::ptr::null;
use ::std::ptr::NonNull;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::OsStr;
use ::std::mem::uninitialized;
use ::std::num::ParseIntError;
use ::std::slice::from_raw_parts;
use ::serde::*;
use ::serde::de;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::de::Error as DeserializerError;
use ::serde::ser;
use ::serde::ser::Serialize;
use ::serde::ser::Serializer;


/// PCI bus.
pub mod pci;


include!("DpdkBus.rs");
include!("DpdkDevice.rs");
include!("DpdkDeviceArguments.rs");
include!("DpdkDeviceMemoryResource.rs");
include!("DpdkDeviceMemoryResources.rs");
include!("DpdkDriver.rs");
include!("NetworkInterfaceName.rs");
