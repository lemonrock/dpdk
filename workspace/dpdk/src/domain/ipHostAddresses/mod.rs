// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::domain::ipNetworkAddresses::*;
use ::libc::in_addr;
use ::libc::in6_addr;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::ptr::copy_nonoverlapping;


include!("IpV4HostAddress.rs");
include!("IpV4HostAddressEx.rs");
include!("ipv4HostAddressFromNumbers.rs");
include!("IpV4HostAddressOctets.rs");
include!("ipV4HostAddressToMappedIpV6HostAddress.rs");
include!("IpV6HostAddress.rs");
include!("IpV6HostAddressEx.rs");
include!("SizeOfIpV4HostAddress.rs");
include!("SizeOfIpV6HostAddress.rs");
