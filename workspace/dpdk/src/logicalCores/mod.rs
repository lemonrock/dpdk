// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::arrayvec::ArrayVec;
use ::const_cstr_fork::ConstCStr;
use ::dpdk_sys::*;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::FILE;
use ::libc::int32_t;
use ::libc_extra::stderr;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::collections::HashMap;
use ::std::fs::File;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::BufReader;
use ::std::io::BufRead;
use ::std::mem::forget;
use ::std::mem::uninitialized;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::null;
use ::syscall_alt::constants::NegativeE;
use ::dpdk_unix::assert_effective_user_id_is_root;
use ::dpdk_unix::PathExt;
use ::dpdk_unix::HugePageSize;
use ::dpdk_unix::android_linux::memory_statistics::MemoryStatisticsParseError;
use ::dpdk_unix::android_linux::memory_statistics::MemoryStatistics;
use ::libc_extra::ffi::callbacks::Callback1;
use ::libc_extra::ffi::callbacks::MutableCallback1;
use ::logicalCores::active::Active;
use ::logicalCores::active::ListParseError;
use ::logicalCores::active::LogicalCoresActive;
use ::logicalCores::active::NumaSocketsActive;
use ::memory::DpdkAllocatedMemory;
use ::rust_extra::powersOfTwo::AsU32;
use ::rust_extra::powersOfTwo::PowerOfTwoThirtyTwoBit;


pub mod active;
pub mod discovery;
pub mod receiveTransmitQueuePair;


include!("AnyNumaSocketId.rs");
include!("HugePageAllocationStrategy.rs");
include!("LogicalCore.rs");
include!("LogicalCorePowerManagement.rs");
include!("NonNumaMemory.rs");
include!("NumaNodesData.rs");
include!("NumaSocketId.rs");
include!("NumaSocketMap.rs");
include!("NumaNodeStatisticName.rs");
include!("NumaNodeStatistics.rs");
include!("SlaveLogicalCoreTask.rs");
