// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_sys;
extern crate libc;


/// Packet matchers, called, confusingly, flow items, by DPDK.
pub mod packet_matchers;


use ::dpdk_sys::*;
use ::libc::memcmp;
use ::std::cmp::Ordering;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::uninitialized;
use ::std::ptr::copy_nonoverlapping;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;


include!("generic_clone.rs");
include!("generic_compare.rs");
include!("generic_equals.rs");
include!("generic_hash.rs");
