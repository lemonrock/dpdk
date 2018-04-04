// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::*;
use ::libc::c_uint;
use ::libc::FILE;
use ::libc_extra::stderr;
use ::rust_extra::likely;
use ::std::marker::PhantomData;
use ::syscall_alt::constants::NegativeE;
use ::libc_extra::ffi::isTrue;
use ::ringQueues::consumers::Consumer;
use ::ringQueues::consumers::InefficientConsumer;
use ::ringQueues::lifecycles::Lifecycle;
use ::ringQueues::lifecycles::GloballyCreatedLifecycle;
use ::ringQueues::lifecycles::GloballyRetrievedLifecycle;
use ::ringQueues::producers::Producer;
use ::ringQueues::producers::InefficientProducer;


pub mod lifecycles;
pub mod producers;
pub mod consumers;


include!("RingQueueProducerConsumerVariant.rs");
include!("RingQueue.rs");
include!("GloballyCreatedInefficentRingQueue.rs");
include!("GloballyRetrievedInefficentRingQueue.rs");
