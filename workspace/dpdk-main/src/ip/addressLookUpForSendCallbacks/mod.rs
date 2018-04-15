// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::arrayvec::ArrayVec;
use ::dpdk::dpdk_sys::*;
use ::dpdk::domain::ethernet::*;
use ::dpdk::domain::internet_protocol::*;
use ::dpdk::domain::internet_protocol::*::*;
use ::dpdk::domain::internet_protocol::*;
use ::dpdk::domain::layer4::*;
use ::dpdk::domain::virtual_lans::*;
use ::dpdk::ethernetPorts::EthernetPort;
use ::dpdk::ethernetPorts::EthernetPortIdentifier;
use ::dpdk::ethernetPorts::MaximumTransmissionUnitSizeInBytes;
use ::dpdk::ethernetPorts::MediaAccessControlAddress;
use ::dpdk::ethernetPorts::QueueIdentifier;
use ::dpdk::logicalCores::NumaSocketId;
use ::dpdk::longestPrefixMatch::*;
use ::dpdk::tldk::AddressLookUpForSendCallback;
use ::libc::in_addr;
use ::libc::in6_addr;
use ::internet_protocol::state::ArpCache;
use ::rust_extra::unlikely;
use ::std::borrow::BorrowMut;
use ::std::cell::RefCell;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::hash::Hash;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::null_mut;
use ::std::rc::Rc;
use ::syscall_alt::constants::NegativeE;


include!("IpV4AddressLookUpForSendCallback.rs");
include!("IpV4Route.rs");
include!("IpV4RoutingTable.rs");
include!("NeverRouteAddressLookUpForSendCallback.rs");
