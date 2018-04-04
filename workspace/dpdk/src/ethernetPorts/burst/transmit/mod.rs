// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::rte_mbuf;
use ::ethernetPorts::*;
use ::libc::c_void;
use ::packetBuffers::rte_mbufEx;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::mem::zeroed;
use ::std::mem::uninitialized;
use ::std::ptr::copy;
use ::tldk::layer4::devices::Device;
use ::tldk::layer4::devices::TcpDevice;
use ::tldk::layer4::devices::UdpDevice;


include!("TransmitBurst.rs");
include!("TransmitBurstFunction.rs");
include!("TransmitBurstFunctionData.rs");
include!("TransmitBurstQueue.rs");
