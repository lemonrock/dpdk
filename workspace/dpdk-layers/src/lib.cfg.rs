// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_core;
extern crate dpdk_sys;
extern crate dpdk_time;
extern crate libc;
#[macro_use] extern crate likely;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate syscall_alt;
// TODO: Suspended as upstream crate needs changes.
// extern crate treebitmap;


use self::ethernet::*;
use self::internet_protocol::*;
use self::internet_protocol::mask_bits::*;
use self::internet_protocol::packet_reassembly::*;
use self::layer4::*;
use self::layer4::internet_control_message_protocol::*;
use self::packet_processing::*;
use self::packet_processing::PacketProcessingDropReason::*;
use self::virtual_lans::*;
use ::arrayvec::ArrayVec;
use ::dpdk_core::*;
use ::dpdk_core::print_information::*;
use ::dpdk_sys::*;
use ::dpdk_time::Cycles;
use ::dpdk_time::Hertz;
use ::dpdk_time::Seconds;
use ::libc::*;
use serde::de;
use serde::de::Deserialize;
// TODO: Suspended as upstream crate needs changes.
// use serde::de::DeserializeOwned;
use serde::de::Deserializer;
use serde::de::Visitor;
use serde::ser::Serialize;
use serde::ser::Serializer;
use ::std::cmp::Ordering;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::err;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::transmute_copy;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::num::TryFromIntError;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::null;
use ::std::ptr::NonNull;
use ::std::rc::Rc;
use ::std::str::SplitN;
use ::syscall_alt::constants::NegativeE;
use ::syscall_alt::constants::E;
// TODO: Suspended as upstream crate needs changes.
// use treebitmap::IpLookupTable;


include!("drop.rs");
include!("unsupported.rs");


/// Internet Protocol (IP) versions 4 and 6.
pub mod ip;


/// Packet processing.
pub mod packet_processing;


include!("Configuration.rs");
include!("InternetProtocolVersion4PacketProcessing.rs");
include!("InternetProtocolVersion6PacketHeaderProcessing.rs");
include!("InternetProtocolVersion6PacketProcessing.rs");
