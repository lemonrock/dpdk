// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate arrayvec;
#[macro_use] extern crate const_cstr_fork;
extern crate dpdk_sys;
extern crate dpdk_unix;
extern crate indexmap;
#[macro_use] extern crate lazy_static;
extern crate libc;
extern crate libc_extra;
extern crate libnuma_sys;
#[macro_use] extern crate likely;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate syscall_alt;


use self::power::*;
use self::print_information::*;
use ::arrayvec::ArrayVec;
use ::const_cstr_fork::ConstCStr;
use ::dpdk_sys::*;
use ::dpdk_unix::*;
use ::dpdk_unix::android_linux::linux_kernel_modules::LinuxKernelModulesList;
use ::dpdk_unix::android_linux::resource_limits::ResourceName;
use ::dpdk_unix::memory_information::MemoryInformation;
use ::dpdk_unix::memory_information::MemoryInformationParseError;
use ::dpdk_unix::signals::SignalNumber;
use ::indexmap::set::IndexSet;
use ::libc::*;
use ::libc_extra::unix::stdio::open_memstream;
use ::libc_extra::unix::stdio::stderr;
use ::libc_extra::unix::stdio::stdout;
use ::libc_extra::unix::string::strsignal;
use ::libnuma_sys::numa_allocate_cpumask;
use ::libnuma_sys::numa_allocate_nodemask;
use ::libnuma_sys::numa_available;
use ::libnuma_sys::numa_bitmask_free;
use ::libnuma_sys::numa_bitmask_isbitset;
use ::libnuma_sys::numa_distance;
use ::libnuma_sys::numa_node_to_cpus;
use ::libnuma_sys::numa_num_possible_cpus;
use ::libnuma_sys::numa_num_possible_nodes;
use ::std::any::Any;
use ::std::collections::BTreeMap;
use ::std::collections::BTreeSet;
use ::std::collections::HashSet;
use ::std::collections::HashMap;
use ::std::cmp::min;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::hash::Hash;
use ::std::io;
use ::std::marker::PhantomData;
#[allow(unused_imports)] use ::std::os::unix::io::IntoRawFd;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::catch_unwind;
use ::std::path::Path;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::ptr::NonNull;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::sync::Arc;
use ::std::sync::Once;
use ::std::sync::ONCE_INIT;
use ::std::time::Duration;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::spin_loop_hint;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::SeqCst;
use ::syscall_alt::PosixErrorNumber;
use ::syscall_alt::constants::NegativeE;


/// CPU power management.
pub mod power;


#[doc(hidden)]
#[allow(non_snake_case)]
pub mod E_RTE;


/// Print information helpers.
pub mod print_information;


include!("AllLogicalCoreIterator.rs");
include!("BusyPollBehaviour.rs");
include!("BusyPollingLogicalCoreFunction.rs");
include!("DeviceDriverName.rs");
include!("DeviceName.rs");
include!("DpdkAllocatedMemory.rs");
include!("DpdkProcess.rs");
include!("EssentialKernelModule.rs");
include!("EssentialKernelModulesToUnload.rs");
include!("HugePageAllocationStrategy.rs");
include!("KiloBytes.rs");
include!("LogicalCore.rs");
include!("LogicalCoreChoice.rs");
include!("MachineOrNumaNodes.rs");
include!("MegaBytes.rs");
include!("Memory.rs");
include!("NumaNode.rs");
include!("NumaNodeChoice.rs");
include!("PerMyriad.rs");
include!("PointerExt.rs");
include!("Service.rs");
include!("ServiceFunction.rs");
include!("ShouldFunctionTerminate.rs");
include!("SlaveLogicalCoreFunction.rs");
include!("SlaveLogicalCoreIterator.rs");
include!("SysLog.rs");
include!("VirtualFunctionIoInterruptMode.rs");
