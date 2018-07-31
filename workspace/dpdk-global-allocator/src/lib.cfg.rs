// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_core;
extern crate dpdk_sys;
#[macro_use] extern crate likely;
extern crate linked_list_allocator;


use ::dpdk_core::*;
use ::dpdk_sys::*;
use ::linked_list_allocator::LockedHeap;
use ::std::alloc::GlobalAlloc;
use ::std::alloc::Layout;
use ::std::cmp::min;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::SeqCst;


include!("HybridGlobalAllocator.rs");
