// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_core;
extern crate dpdk_sys;
#[macro_use] extern crate likely;
#[macro_use] extern crate quick_error;
extern crate network_collections;


use ::dpdk_core::*;
use ::dpdk_sys::*;
use ::network_collections::NonNullUnifiedArrayVecAndVec;
use ::std::ffi::CStr;
use ::std::ptr::NonNull;


include!("CouldNotInsertPacketBufferForReordering.rs");
include!("PacketReorderer.rs");
