// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_core;
extern crate dpdk_sys;
extern crate libc;
extern crate serde;
#[macro_use] extern crate serde_derive;


use ::dpdk_core::*;
use ::dpdk_core::print_information::*;
use ::dpdk_sys::*;
use ::libc::*;
use ::std::cell::UnsafeCell;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::ops::Deref;
use ::std::ops::Sub;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::rc::Rc;
use ::std::sync::Arc;


include!("AlarmClock.rs");
include!("AlarmClockCallback.rs");
include!("Cycles.rs");
include!("Hertz.rs");
include!("Milliseconds.rs");
include!("Microseconds.rs");
include!("Nanoseconds.rs");
include!("SmartPointer.rs");
include!("Timer.rs");
include!("TimerCallback.rs");
include!("TimerProgressEngine.rs");
