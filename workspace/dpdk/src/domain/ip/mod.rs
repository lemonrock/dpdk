// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::domain::ipHostAddresses::SizeOfIpV6HostAddress;
use ::domain::ipHostAddresses::SizeOfIpV4HostAddress;
use ::dpdk_sys::ipv4_hdr;
use ::dpdk_sys::ipv6_hdr;
use ::std::mem::size_of;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use ::domain::layer4::Layer4Protocol;


include!("DifferentiatedServiceCodePoint.rs");
include!("ExplicitCongestionNotification.rs");
include!("IpProtocol.rs");
include!("IpV4AndOrIpV6.rs");
include!("TrafficClass.rs");
