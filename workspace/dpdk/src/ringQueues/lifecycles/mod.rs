// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::*;
use ::libc::c_uint;
use ::libc::FILE;
use ::libc_extra::stderr;
use ::rust_extra::unlikely;
use ::std::ffi::CString;
use ::syscall_alt::constants::E;
use ::E_RTE;
use ::logicalCores::NumaSocketId;
use ::logicalCores::AnyNumaSocketId;
use ::ringQueues::RingQueue;
use ::ringQueues::RingQueueProducerConsumerVariant;
use ::ringQueues::consumers::InefficientConsumer;
use ::ringQueues::producers::InefficientProducer;


include!("Lifecycle.rs");
include!("GloballyCreatedLifecycle.rs");
include!("GloballyRetrievedLifecycle.rs");
