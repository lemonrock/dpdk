// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_bus;
extern crate dpdk_sys;
extern crate either;
extern crate libc;
#[macro_use] extern crate likely;
extern crate network_collections;
extern crate serde;
#[macro_use] extern crate serde_derive;


use self::link_status::*;
use self::queue_identifiers::*;
use ::dpdk_bus::pci::*;
use ::dpdk_sys::*;
pub use ::either::*;
use ::libc::*;
use ::network_collections::Array40;
use ::network_collections::Array52;
use ::network_collections::NonNullUnifiedArrayVecAndVec;
use ::std::borrow::Cow;
use ::std::cmp::min;
use ::std::convert::TryFrom;
use ::std::iter::Step;
use ::std::mem::replace;
use ::std::mem::uninitialized;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use ::std::ptr::NonNull;


/// Link status.
pub mod link_status;


/// Queue identifiers.
pub mod queue_identifiers;


/// Receive side scaling.
pub mod receive_side_scaling;


include!("EthernetPortIdentifier.rs");
include!("ReceiveBurst.rs");
include!("TransmitBurst.rs");
