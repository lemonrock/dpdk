// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::domain::ipHostAddresses::*;
use ::domain::maskBits::*;
use ::serde::Deserialize;
use ::serde::de::Deserializer;
use ::serde::Serialize;
use ::serde::ser::Serializer;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;


include!("IpNetworkAddress.rs");
include!("IpV4NetworkAddress.rs");
include!("IpV6NetworkAddress.rs");
