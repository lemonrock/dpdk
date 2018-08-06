// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_core;
extern crate dpdk_sys;
extern crate libc;
#[macro_use] extern crate likely;
extern crate network_internet_protocol;


use ::dpdk_core::*;
use ::dpdk_sys::*;
use ::libc::*;
use ::network_internet_protocol::*;
use ::network_internet_protocol::version_4::*;
use ::network_internet_protocol::version_6::*;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::uninitialized;
use ::std::ptr::NonNull;


include!("HostAddressesSetConfiguration.rs");
include!("InternetProtocolVersion4LongestPrefixMatchTable.rs");
include!("InternetProtocolVersion6LongestPrefixMatchTable.rs");
include!("LongestPrefixMatchTable.rs");
include!("RoutingTableKey.rs");
