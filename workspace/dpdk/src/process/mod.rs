// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::const_cstr_fork::ConstCStr;
use ::devices::virtualDevices::*;
use ::devices::virtualDevices::cryptoVirtualDevices::*;
use ::devices::virtualDevices::netVirtualDevices::*;
use ::dpdk_unix::helpers::getProgramName;
use ::dpdk_unix::strings::osStrToCString;
use ::dpdk_sys::rte_intr_mode;
use ::dpdk_sys::rte_proc_type_t;
use ::dpdk_unix::strings::pathToCString;
use ::libc::c_char;
use ::libc::c_int;
use ::libc_extra::ffi::isTrue;
use ::libc_extra::ffi::arguments::VecArguments;
use ::logicalCores::MaximumNumaSockets;
use ::logicalCores::discovery::NumaSockets;
use ::pci::DeviceAddress;
use ::pci::DeviceListColour;
use ::rust_extra::likely;
use ::rust_extra::u31;
use ::std::collections::HashSet;
use ::std::ffi::CString;
use ::std::ffi::OsStr;
use ::std::ffi::OsString;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::null;
use ::std::ptr::null_mut;


include!("DpdkRteInitData.rs");
include!("hasHugepages.rs");
include!("HugePageFilePathInformation.rs");
include!("isPrimaryProcessAlive.rs");
include!("MemoryChannels.rs");
include!("MemoryLimits.rs");
include!("MemoryRanks.rs");
include!("ProcessType.rs");
include!("VfioInterruptMode.rs");
