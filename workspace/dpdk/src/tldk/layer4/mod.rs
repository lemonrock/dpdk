// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::domain::ip::IpV4AndOrIpV6;
use ::domain::ipHostAddresses::*;
use ::domain::layer4::Layer4Port;
use ::domain::layer4::Layer4Protocol;
use ::dpdk_sys::rust_rte_errno;
use ::dpdk_sys::tle_bl_port;
use ::dpdk_sys::tle_ctx;
use ::dpdk_sys::tle_ctx_param;
use ::dpdk_sys::tle_dest;
use ::dpdk_sys::tle_dev_param;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::in_addr;
use ::libc::in6_addr;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::cell::RefCell;
use ::std::collections::HashSet;
use ::std::mem::forget;
use ::std::mem::zeroed;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::panic::catch_unwind;
use ::std::panic::AssertUnwindSafe;
use ::std::ptr::null;
use ::std::rc::Rc;
use ::syscall_alt::constants::E;
use ::syscall_alt::constants::NegativeE;
use ::ethernetPorts::DeviceReceiveOffloadCapabilities;
use ::ethernetPorts::DeviceTransmitOffloadCapabilities;
use ::logicalCores::AnyNumaSocketId;
use ::logicalCores::NumaSocketId;
use ::tldk::layer4::devices::Device;
use ::tldk::layer4::devices::TcpDevice;
use ::tldk::layer4::devices::UdpDevice;

pub mod devices;
pub mod streams;


include!("AddressLookUpForSendCallback.rs");
include!("AddressWithListOfOpenLayer4Ports.rs");
include!("Context.rs");
include!("DeviceConfiguration.rs");
include!("TldkBlockedPortsList.rs");
include!("TcpContext.rs");
include!("UdpContext.rs");
