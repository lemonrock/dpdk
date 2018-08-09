// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate arrayvec;
extern crate dpdk_core;
extern crate dpdk_sys;
#[macro_use] extern crate likely;
extern crate network_address_resolution_protocol;
extern crate network_check_sum;
extern crate network_endian;
extern crate network_ethernet;
extern crate network_internet_protocol;
extern crate network_internet_control_message_protocol;
extern crate serde;
#[macro_use] extern crate serde_derive;


use self::pattern_item_implementations::*;
use ::arrayvec::ArrayVec;
use ::dpdk_core::*;
use ::dpdk_sys::*;
use ::network_address_resolution_protocol::*;
use ::network_check_sum::*;
use ::network_endian::*;
use ::network_ethernet::*;
use ::network_ethernet::virtual_lans::*;
#[allow(unused_imports)] use ::network_internet_protocol::InternetProtocolHostAddress;
use ::network_internet_protocol::version_4::*;
use ::network_internet_protocol::version_6::*;
use ::network_internet_control_message_protocol::version_4::*;
use ::network_internet_control_message_protocol::version_6::*;
use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::de::Error as DeserializerError;
use ::serde::de::MapAccess;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;
use ::std::fmt;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;


include!("custom_deserialize.rs");


/// Pattern item implementations.
pub mod pattern_item_implementations;


include!("FlowRule.rs");
include!("Pattern.rs");
include!("TrafficDirection.rs");
