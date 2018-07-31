// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_core;
extern crate dpdk_sys;
#[macro_use] extern crate likely;
extern crate lock_free_multi_producer_single_consumer_ring_buffer;
extern crate network_collections;


use ::dpdk_core::*;
use ::dpdk_sys::*;
use ::lock_free_multi_producer_single_consumer_ring_buffer::*;
use ::network_collections::ArrayVec;
use ::network_collections::NonNullUnifiedArrayVecAndVec;
use ::network_collections::UnifiedArrayVecAndVec;
use ::std::ffi::CString;
use ::std::mem::transmute;
use ::std::ptr::NonNull;
use ::std::sync::Arc;


include!("PacketDistributorController.rs");
include!("PacketDistributorControllerLoop.rs");
include!("PacketDistributorWorker.rs");
include!("PacketDistributorWorkerIterator.rs");
