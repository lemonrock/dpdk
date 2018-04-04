// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::arrayvec::ArrayVec;
use ::dpdk_sys::rust_rte_errno;
use ::dpdk_sys::rte_mbuf;
use ::dpdk_sys::tle_stream;
use ::dpdk_sys::tle_stream_cb;
use ::dpdk_sys::tle_tcp_stream_addr;
use ::dpdk_sys::tle_tcp_stream_cfg;
use ::dpdk_sys::tle_tcp_stream_param;
use ::dpdk_sys::tle_udp_stream_param;
use ::libc::c_void;
use ::libc::in_addr;
use ::libc::in6_addr;
use ::libc::sockaddr;
use ::libc::sockaddr_storage;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::uninitialized;
use ::std::panic::catch_unwind;
use ::std::panic::AssertUnwindSafe;
use ::std::ptr::null_mut;
use ::syscall_alt::constants::E;
use ::syscall_alt::constants::NegativeE;
use ::tldk::event::Event;
use ::tldk::layer4::AddressLookUpForSendCallback;
use ::tldk::layer4::Context;
use ::tldk::layer4::UdpContext;
use ::tldk::layer4::TcpContext;
use ::domain::layer4::Layer4Protocol;


include!("EdgeTriggeredCallback.rs");
include!("EventNotificationKind.rs");
include!("Stream.rs");
include!("TcpStream.rs");
include!("UdpStream.rs");
