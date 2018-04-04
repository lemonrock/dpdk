// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::rte_mbuf;
use ::dpdk_sys::tle_dev;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::syscall_alt::constants::NegativeE;
use ::domain::layer4::Layer4Protocol;


include!("Device.rs");
include!("PacketProcessor.rs");
include!("TcpDevice.rs");
include!("UdpDevice.rs");
