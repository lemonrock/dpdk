// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::arrayvec::ArrayVec;
use ::dpdk_unix::set_current_thread_name;
use ::dpdk_sys::*;
use ::libc_extra::ffi::callbacks::MutableCallback1;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::rust_extra::arrays::Array;
use ::std::sync::atomic::AtomicU64;
use ::std::mem::zeroed;
use ::std::panic::catch_unwind;
use ::std::panic::AssertUnwindSafe;
use ::std::sync::Arc;
use ::std::sync::Mutex;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering;
use ::std::thread;
use ::ethernetPorts::*;
use ::logicalCores::*;
use ::packetBuffers::PacketBufferPool;


include!("CanContinue.rs");
include!("ExecutionRoutine.rs");
include!("ExecutionRoutineCreator.rs");
include!("ExecutionRoutineCreatorCreator.rs");
include!("ExecutionRoutineGroup.rs");
include!("ReceiveTransmitQueueMemoryConfiguration.rs");
include!("ReceiveTransmitQueuePairSlaveLogicalCoreTask.rs");

// Design is probably wrong
include!("RetaHashStatistics.rs");
