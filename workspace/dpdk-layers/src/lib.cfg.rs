// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_use] extern crate arrayref;
extern crate arrayvec;
extern crate dpdk_core;
#[macro_use] extern crate dpdk_likely;
extern crate dpdk_sys;
extern crate dpdk_time;
extern crate hyper_thread_random;
extern crate libc;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate syscall_alt;
extern crate treebitmap;


use self::ethernet::*;
use self::address_resolution_protocol::*;
use self::internet_protocol::*;
use self::internet_protocol::longest_prefix_matching::*;
use self::internet_protocol::mask_bits::*;
use self::layer4::*;
use self::layer4::internet_control_message_protocol::*;
use self::virtual_lans::*;
use ::hyper_thread_random::generate_hyper_thread_safe_random_u64;
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
use serde::de::DeserializeOwned;
use serde::de::Deserializer;
use serde::de::Visitor;
use serde::ser::Serialize;
use serde::ser::Serializer;
use ::std::cmp::Ordering;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::transmute_copy;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::null;
use ::std::ptr::NonNull;
use ::std::slice::from_raw_parts;
use ::std::str::SplitN;
use ::syscall_alt::constants::NegativeE;
use ::syscall_alt::constants::E;
use treebitmap::IpLookupTable;


macro_rules! finish
{
	($packet: ident) =>
	{
		{
			$packet.free_direct_contiguous_packet();
			return
		}
	}
}


/// Address Resolution Protocol (ARP).
pub mod address_resolution_protocol;


/// Ethernet.
pub mod ethernet;


/// Internet Protocol (IP) versions 4 and 6.
pub mod internet_protocol;


/// Layer 4 protocols (TCP, UDP).
pub mod layer4;


/// Virtual LANs (VLANs).
pub mod virtual_lans;


include!("NetworkByteOrderEndianU16.rs");
include!("NonNullUnifiedArrayVecAndVec.rs");
include!("PacketBuffer.rs");
include!("PacketBufferPool.rs");
include!("PacketProcessingConfiguration.rs");
include!("UnifiedArrayVecAndVec.rs");
