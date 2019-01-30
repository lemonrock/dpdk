// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_bus;
extern crate dpdk_core;
extern crate dpdk_global_allocator;
extern crate dpdk_sys;
extern crate dpdk_time;
extern crate dpdk_unix;
extern crate dpdk_virtual_devices;
extern crate libc;
#[macro_use] extern crate maplit;
extern crate serde;
#[macro_use] extern crate serde_derive;


use ::dpdk_bus::pci::IndirectPciDeviceIdentifier;
use ::dpdk_bus::pci::PciDevice;
use ::dpdk_bus::pci::PciKernelDriver;
use ::dpdk_core::*;
use ::dpdk_core::memory::*;
use ::dpdk_core::power::*;
use ::dpdk_global_allocator::HybridGlobalAllocator;
use ::dpdk_sys::*;
use ::dpdk_time::*;
use ::dpdk_unix::*;
use ::dpdk_unix::logging::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::android_linux::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::android_linux::capabilities::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::android_linux::mounts::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::android_linux::process_control::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::android_linux::resource_limits::*;
use ::dpdk_unix::signals::*;
use ::dpdk_virtual_devices::*;
use ::dpdk_virtual_devices::net_virtual_devices::*;
use ::libc::*;
use ::std::collections::BTreeMap;
use ::std::collections::BTreeSet;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::env::current_exe;
use ::std::env::set_var;
use ::std::ffi::CString;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::catch_unwind;
use ::std::panic::resume_unwind;
use ::std::panic::set_hook;
use ::std::panic::take_hook;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::null_mut;
use ::std::sync::Arc;


include!("Arguments.rs");
include!("dpdk_provided_kernel_modules_path.rs");
include!("DpdkConfiguration.rs");
include!("MasterLoop.rs");
include!("MasterLoopConfiguration.rs");
include!("PciNetDevicesConfiguration.rs");
include!("ProcessType.rs");
